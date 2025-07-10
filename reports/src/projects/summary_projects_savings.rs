use std::{io::Cursor, sync::Arc};

use axum::{body::Body, http::{header, Response}, response::IntoResponse, Extension};
use genpdf::{elements::{Break, LinearLayout, Paragraph, TableLayout}, style, Alignment, Document, Element, Margins, Position};
use num_format::{Locale, ToFormattedString};
use services::{db_connection::DB, redis::Redis, response::reports::ResponseSummaryProjectSavingsReportOverview};

use crate::helpers::{app_icon_images, city_images, doc_title, font_family, PaperSize};

pub async fn reports_summary_projects_savings (
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {
    
    let mut redis = Redis::new("summary_project_savings_report".to_string(), db.redis_connection.clone());

    let sse = Arc::new(db.sse_connection.clone());    


    let get_redis_value = redis.get_value().await;

    if get_redis_value.is_none() {
        // SSE EVENTS
        let clone_sse = sse.clone();
        tokio::spawn(async move {
            clone_sse.send_progress_update("projects-summary-savings", 0, "No Data Found");
        });

        return Response::default();
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary-savings", 0, "Preparing data information");
    });

    let get_parse_projects: ResponseSummaryProjectSavingsReportOverview = serde_json::from_str(&get_redis_value.unwrap()).unwrap();

    let get_project = get_parse_projects.data;

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary-savings", 1, "Preparing report document");
    });

    let mut doc = Document::new(font_family());
    doc.set_title("City Engineer's Office - Management Information System");
    doc.set_paper_size(PaperSize::A4LandScape.size());


    doc.push(city_images().with_position(Position::new(40, 10)));
    doc.push(app_icon_images().with_position(Position::new(25, 10)));
    doc.push(doc_title());

    // Project Information
    let reports_title = Paragraph::new("Summary Project Savings Report")
        .aligned(Alignment::Center)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(17))
        .padded(Margins::trbl(5, 0, 0, 0));


    // Project Summary Table
    let mut table_projects_summary = TableLayout::new(vec![2,1,1,1,2]);

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
            .styled(style::Style::new().with_font_size(7)).padded(Margins::trbl(0, 0, 0, 5)));

        row_project_summary_head.push_element(Paragraph::new("Contractor")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Status")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push_element(Paragraph::new("Savings Calculation")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(7)));

        row_project_summary_head.push().unwrap();
    }

    for projects in get_project {
        let mut row_project_summary_data = table_projects_summary.row();

        row_project_summary_data.push_element(Paragraph::new(projects.project_name)
            .padded(Margins::trbl(2, 0, 0, 2))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects.project_code)
            .padded(Margins::trbl(2, 0, 0, 5))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects.contractor_name.unwrap_or("-".to_string()))
            .padded(Margins::trbl(2, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );
        row_project_summary_data.push_element(Paragraph::new(projects.project_status.unwrap_or("-".to_string()))
            .padded(Margins::trbl(2, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );

        let mut main_savings_layout = LinearLayout::vertical();

        main_savings_layout.push(Paragraph::new(format!("ABC Cost: {}", projects.approved_budget_contract.unwrap_or(0).to_formatted_string(&Locale::en)))
            .padded(Margins::trbl(2, 0, 0, 0))
            .styled(style::Style::new().with_font_size(7))
        );

        let mut make_calculation_table = TableLayout::new(vec![2, 1, 2, 1, 2]);
        let mut row_calculation_table = make_calculation_table.row();

        let mut make_appropriation_layout = LinearLayout::vertical();
        make_appropriation_layout.push(Paragraph::new("Appropriation")
            .styled(style::Style::new().with_font_size(7))
        );
        make_appropriation_layout.push(Paragraph::new(projects.appropriation.unwrap_or(0).to_formatted_string(&Locale::en))
            .styled(style::Style::new().with_font_size(7))
        );

        let mut make_contract_cost_layout = LinearLayout::vertical();
        make_contract_cost_layout.push(Paragraph::new("Contract Cost")
            .styled(style::Style::new().with_font_size(7))
        );
        make_contract_cost_layout.push(Paragraph::new(projects.contract_cost.unwrap_or(0).to_formatted_string(&Locale::en))
            .styled(style::Style::new().with_font_size(7))
        );

        let mut make_savings_layout = LinearLayout::vertical();
        make_savings_layout.push(Paragraph::new("Savings")
            .styled(style::Style::new().with_font_size(7))
        );
        make_savings_layout.push(Paragraph::new(projects.savings.to_formatted_string(&Locale::en))
            .styled(style::Style::new().with_font_size(7))
        );

        row_calculation_table.push_element(make_appropriation_layout);
        row_calculation_table.push_element(Paragraph::new("-"));
        row_calculation_table.push_element(make_contract_cost_layout);
        row_calculation_table.push_element(Paragraph::new("="));
        row_calculation_table.push_element(make_savings_layout);

        row_calculation_table.push().unwrap();

        main_savings_layout.push(make_calculation_table);

        row_project_summary_data.push_element(main_savings_layout);
        row_project_summary_data.push().unwrap();
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary-savings", 2, "Compiling project summary informations");
    });

    doc.push(reports_title);
    doc.push(Break::new(1));
    doc.push(table_projects_summary);

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary-savings", 3, "Generating PDF file");
    });

    let mut buffer = Cursor::new(Vec::new());
    doc.render(&mut buffer).expect("Failed to render PDF Document");

    let pdf_bytes = buffer.into_inner();

    let projects_file_name = format!("projects-summary-savings.pdf");
    let attachment = format!("attachment; filename=\"{}\"", projects_file_name);

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-summary-savings", 4, "PDF Report Completed");
    });

    Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, attachment)
        .header(header::CONTENT_LENGTH, pdf_bytes.len())
        .body(Body::from(pdf_bytes))
        .unwrap()
}