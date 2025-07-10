use std::{io::Cursor, sync::Arc};

use axum::{body::Body, http::{header, Response}, response::IntoResponse, Extension};
use genpdf::{elements::{Break, Paragraph, TableLayout}, style, Alignment, Document, Element, Margins, Position};
use services::{db_connection::DB, redis::Redis, response::reports::ResponseSummarySlippageReportOverview};

use crate::helpers::{app_icon_images, city_images, doc_title, font_family, PaperSize};

pub async fn reports_slippage_projects (
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("summary_slippage_report".to_string(), db.redis_connection.clone());

    let sse = Arc::new(db.sse_connection.clone());    

    let get_redis_value = redis.get_value().await;

    if get_redis_value.is_none() {
        // SSE EVENTS
        let clone_sse = sse.clone();
        tokio::spawn(async move {
            clone_sse.send_progress_update("projects-slippage-report", 0, "No Data Found");
        });

        return Response::default();
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-slippage-report", 0, "Preparing data information");
    });

    let get_parse_projects: ResponseSummarySlippageReportOverview = serde_json::from_str(&get_redis_value.unwrap()).unwrap();

    let get_projects = get_parse_projects.data;

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-slippage-report", 1, "Preparing report document");
    });

    let mut doc = Document::new(font_family());
    doc.set_title("City Engineer's Office - Management Information System");
    doc.set_paper_size(PaperSize::A4LandScape.size());


    doc.push(city_images().with_position(Position::new(40, 10)));
    doc.push(app_icon_images().with_position(Position::new(25, 10)));
    doc.push(doc_title());

    // Project Information
    let reports_title = Paragraph::new("Slippage Report")
        .aligned(Alignment::Center)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(17))
        .padded(Margins::trbl(5, 0, 0, 0));

    // Project Summary Table
    let mut table_projects_summary = TableLayout::new(vec![2,1,1,1,1,1,2]);

    // ROW HEAD
    {
        let mut row_project_summary_head = table_projects_summary.row();

        row_project_summary_head.push_element(Paragraph::new("Project Name")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)).padded(Margins::trbl(0, 0, 0, 2)));

        row_project_summary_head.push_element(Paragraph::new("Contractor")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)).padded(Margins::trbl(0, 0, 0, 5)));

        row_project_summary_head.push_element(Paragraph::new("Taker")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Start Date")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Target Date")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("No. of days lapsed")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Remarks")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push().unwrap();
    }

    for project in get_projects {
        let mut row_project_summary_data = table_projects_summary.row();

        row_project_summary_data.push_element(Paragraph::new(project.project_name)
            .padded(Margins::trbl(1, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(project.contractor_name)
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(project.taker_name)
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(project.start_date)
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(project.target_date)
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(format!("{} days", project.days_lapse))
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(project.remarks)
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );

        row_project_summary_data.push().unwrap();
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-slippage-report", 2, "Compiling project slippage informations");
    });

    doc.push(reports_title); 
    doc.push(Break::new(1));
    doc.push(table_projects_summary);

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-slippage-report", 3, "Generating PDF file");
    });

    let mut buffer = Cursor::new(Vec::new());
    doc.render(&mut buffer).expect("Failed to render PDF Document");

    let pdf_bytes = buffer.into_inner();

    let projects_file_name = format!("projects-slippage.pdf");
    let attachment = format!("attachment; filename=\"{}\"", projects_file_name);

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-slippage-report", 4, "PDF Report Completed");
    });

    Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, attachment)
        .header(header::CONTENT_LENGTH, pdf_bytes.len())
        .body(Body::from(pdf_bytes))
        .unwrap()
}