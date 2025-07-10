use std::{io::Cursor, sync::Arc};

use axum::{body::Body, http::{header, Response}, response::IntoResponse, Extension};
use genpdf::{elements::{Break, Paragraph, TableLayout}, style, Alignment, Document, Element, Margins, Position, Size};
use num_format::{Locale, ToFormattedString};
use services::{db_connection::DB, redis::Redis, response::reports::ResponseSummaryListOfProjectsOverview};

use crate::helpers::{app_icon_images, city_images, doc_title, font_family};

pub async fn reports_summary_list_of_projects(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("summary_list_of_projects".to_string(), db.redis_connection.clone());

    let sse = Arc::new(db.sse_connection.clone());    

    let get_redis_value = redis.get_value().await;

    if get_redis_value.is_none() {
        // SSE EVENTS
        let clone_sse = sse.clone();
        tokio::spawn(async move {
            clone_sse.send_progress_update("projects-summary", 0, "No Data Found");
        });

        return Response::default();
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary", 0, "Preparing data information");
    });


    let get_parse_projects: ResponseSummaryListOfProjectsOverview = serde_json::from_str(&get_redis_value.unwrap()).unwrap();

    let get_projects = get_parse_projects.summary_list_of_reports;

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary", 1, "Preparing report document");
    });

    let mut doc = Document::new(font_family());
    doc.set_title("City Engineer's Office - Management Information System");
    doc.set_paper_size(Size::new(297.0, 210.0));


    doc.push(city_images().with_position(Position::new(40, 10)));
    doc.push(app_icon_images().with_position(Position::new(25, 10)));
    doc.push(doc_title());

    // Project Information
    let reports_title = Paragraph::new("Summary list of projects")
        .aligned(Alignment::Center)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(17))
        .padded(Margins::trbl(5, 0, 0, 0));



    // Projects Summary Table
    let mut table_projects_summary = TableLayout::new(vec![5,3,3,3,3,3,3,2,3,1,4]);

    // ROW HEAD
    {
        let mut row_project_summary_head = table_projects_summary.row();

        row_project_summary_head.push_element(Paragraph::new("Project Name")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)).padded(Margins::trbl(0, 0, 0, 2)));

        row_project_summary_head.push_element(Paragraph::new("Code")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Appropriation")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Bid Date")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));
        
        row_project_summary_head.push_element(Paragraph::new("Contract Cost")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Status")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Contractor")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("% Accom")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Taker")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Year")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Remarks")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push().unwrap();
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary", 2, "Compiling project summary informations");
    });

    // Project Data
    for projects_data in get_projects {
        let mut row_project_summary_data = table_projects_summary.row();

        row_project_summary_data.push_element(Paragraph::new(projects_data.project_name)
            .padded(Margins::trbl(1, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects_data.project_code)
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects_data.project_appropriation.unwrap_or(0).to_formatted_string(&Locale::en))
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects_data.project_bid_date.unwrap_or("-".to_string()))
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects_data.project_contract_cost.unwrap_or(0).to_formatted_string(&Locale::en))
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects_data.contractor_name.unwrap_or("-".to_string()))
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects_data.project_status.unwrap_or("-".to_string()))
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects_data.project_accomplished.unwrap_or(0).to_string())
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects_data.project_taker_name.unwrap_or("-".to_string()))
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects_data.project_year.to_string())
            .padded(Margins::trbl(1, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects_data.project_remarks.unwrap_or("-".to_string()))
            .padded(Margins::trbl(1, 2, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push().unwrap();
    }

    doc.push(reports_title); 
    doc.push(Break::new(1));
    doc.push(table_projects_summary);
    
    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary", 3, "Generating PDF file");
    });

    let mut buffer = Cursor::new(Vec::new());
    doc.render(&mut buffer).expect("Failed to render PDF Document");

    let pdf_bytes = buffer.into_inner();

    let projects_file_name = format!("projects-summary.pdf");
    let attachment = format!("attachment; filename=\"{}\"", projects_file_name);

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary", 4, "PDF Report Completed");
    });

    Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, attachment)
        .header(header::CONTENT_LENGTH, pdf_bytes.len())
        .body(Body::from(pdf_bytes))
        .unwrap()
}