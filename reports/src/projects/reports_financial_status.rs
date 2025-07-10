use std::{io::Cursor, sync::Arc};

use axum::{body::Body, http::{header, Response}, response::IntoResponse, Extension};
use genpdf::{elements::{Break, Paragraph, TableLayout}, style, Alignment, Document, Element, Margins, Position};
use num_format::{Locale, ToFormattedString};
use services::{db_connection::DB, redis::Redis, response::reports::ResponseSummaryFinancialStatusReportsOverview};

use crate::helpers::{app_icon_images, city_images, doc_title, font_family, PaperSize};

pub async fn reports_financial_status (
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("summary_financial_status_report".to_string(), db.redis_connection.clone());

    let sse = Arc::new(db.sse_connection.clone());    

    let get_redis_value = redis.get_value().await;

    if get_redis_value.is_none() {
        // SSE EVENTS
        let clone_sse = sse.clone();
        tokio::spawn(async move {
            clone_sse.send_progress_update("projects-financial-status-report", 0, "No Data Found");
        });
        return Response::default();
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-financial-status-report", 0, "Preparing data information");
    });
    let get_parse_projects: ResponseSummaryFinancialStatusReportsOverview = serde_json::from_str(&get_redis_value.unwrap()).unwrap();

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-financial-status-report", 1, "Preparing report document");
    });

    let mut doc = Document::new(font_family());
    doc.set_title("City Engineer's Office - Management Information System");
    doc.set_paper_size(PaperSize::A4Portrait.size());

    doc.push(city_images().with_position(Position::new(40, 10)));
    doc.push(app_icon_images().with_position(Position::new(25, 10)));
    doc.push(doc_title());

    // Project Information
    let reports_title = Paragraph::new("Financial status summary report")
        .aligned(Alignment::Center)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(17))
        .padded(Margins::trbl(5, 0, 0, 0));

    // Implemented Title
    let implemented_title = Paragraph::new("Implemented (Preperation, Bidded, Bidding, On-going, Completed)")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))
        .padded(Margins::trbl(0, 0, 0, 5));

    // implemented data
    let get_implemented_data = get_parse_projects.data_implemented;

    // projects
    let mut implemented_no_projects_table = TableLayout::new(vec![1,1]);
    let mut implemented_no_projects_row = implemented_no_projects_table.row();

    implemented_no_projects_row.push_element(Paragraph::new("No. Projects")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    implemented_no_projects_row.push_element(Paragraph::new(get_implemented_data.total_projects.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    implemented_no_projects_row.push().unwrap();

    // appropriation
    let mut implemented_appropriation_table = TableLayout::new(vec![1,1]);
    let mut implemented_appropriation_row = implemented_appropriation_table.row();

    implemented_appropriation_row.push_element(Paragraph::new("Appropriation")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    implemented_appropriation_row.push_element(Paragraph::new(get_implemented_data.total_appropriation.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    implemented_appropriation_row.push().unwrap();

    // Contract Cost
    let mut implemented_contract_cost_table = TableLayout::new(vec![1,1]);
    let mut implemented_contract_cost_row = implemented_contract_cost_table.row();

    implemented_contract_cost_row.push_element(Paragraph::new("Contract Cost")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    implemented_contract_cost_row.push_element(Paragraph::new(get_implemented_data.total_contract_cost.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    implemented_contract_cost_row.push().unwrap();

    // Paid
    let mut implemented_paid_table = TableLayout::new(vec![1,1]);
    let mut implemented_paid_row = implemented_paid_table.row();

    implemented_paid_row.push_element(Paragraph::new("Paid")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    implemented_paid_row.push_element(Paragraph::new(get_implemented_data.total_paid.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    implemented_paid_row.push().unwrap();

    // Balance
    let mut implemented_balance_table = TableLayout::new(vec![1,1]);
    let mut implemented_balance_row = implemented_balance_table.row();

    implemented_balance_row.push_element(Paragraph::new("Balance")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    implemented_balance_row.push_element(Paragraph::new(get_implemented_data.total_balance.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    implemented_balance_row.push().unwrap();

    // Savings
    let mut implemented_savings_table = TableLayout::new(vec![1,1]);
    let mut implemented_savings_row = implemented_savings_table.row();

    implemented_savings_row.push_element(Paragraph::new("Savings")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    implemented_savings_row.push_element(Paragraph::new(get_implemented_data.total_savings.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    implemented_savings_row.push().unwrap();


    // Unimplemented Title
    let unimplemented_title = Paragraph::new("Unimplemented (Not Yet Started)")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))
        .padded(Margins::trbl(0, 0, 0, 5));

    // unimplemented data
    let get_unimplemented_data = get_parse_projects.data_unimplemented;

    // projects
    let mut unimplemented_no_projects_table = TableLayout::new(vec![1,1]);
    let mut unimplemented_no_projects_row = unimplemented_no_projects_table.row();

    unimplemented_no_projects_row.push_element(Paragraph::new("No. Projects")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    unimplemented_no_projects_row.push_element(Paragraph::new(get_unimplemented_data.total_projects.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    unimplemented_no_projects_row.push().unwrap();

    // appropriation
    let mut unimplemented_appropriation_table = TableLayout::new(vec![1,1]);
    let mut unimplemented_appropriation_row = unimplemented_appropriation_table.row();

    unimplemented_appropriation_row.push_element(Paragraph::new("Appropriation")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    unimplemented_appropriation_row.push_element(Paragraph::new(get_unimplemented_data.total_appropriation.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    unimplemented_appropriation_row.push().unwrap();

    // Contract Cost
    let mut unimplemented_contract_cost_table = TableLayout::new(vec![1,1]);
    let mut unimplemented_contract_cost_row = unimplemented_contract_cost_table.row();

    unimplemented_contract_cost_row.push_element(Paragraph::new("Contract Cost")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    unimplemented_contract_cost_row.push_element(Paragraph::new(get_unimplemented_data.total_contract_cost.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    unimplemented_contract_cost_row.push().unwrap();

    // Paid
    let mut unimplemented_paid_table = TableLayout::new(vec![1,1]);
    let mut unimplemented_paid_row = unimplemented_paid_table.row();

    unimplemented_paid_row.push_element(Paragraph::new("Paid")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    unimplemented_paid_row.push_element(Paragraph::new(get_unimplemented_data.total_paid.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    unimplemented_paid_row.push().unwrap();

    // Balance
    let mut unimplemented_balance_table = TableLayout::new(vec![1,1]);
    let mut unimplemented_balance_row = unimplemented_balance_table.row();

    unimplemented_balance_row.push_element(Paragraph::new("Balance")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    unimplemented_balance_row.push_element(Paragraph::new(get_unimplemented_data.total_balance.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    unimplemented_balance_row.push().unwrap();

    // Savings
    let mut unimplemented_savings_table = TableLayout::new(vec![1,1]);
    let mut unimplemented_savings_row = unimplemented_savings_table.row();

    unimplemented_savings_row.push_element(Paragraph::new("Savings")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    unimplemented_savings_row.push_element(Paragraph::new(get_unimplemented_data.total_savings.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    unimplemented_savings_row.push().unwrap();


    // Suspended Title
    let suspended_title = Paragraph::new("Suspended")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))
        .padded(Margins::trbl(0, 0, 0, 5));

    // suspended data
    let get_suspended_data = get_parse_projects.data_suspended;

    // projects
    let mut suspended_no_projects_table = TableLayout::new(vec![1,1]);
    let mut suspended_no_projects_row = suspended_no_projects_table.row();

    suspended_no_projects_row.push_element(Paragraph::new("No. Projects")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    suspended_no_projects_row.push_element(Paragraph::new(get_suspended_data.total_projects.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    suspended_no_projects_row.push().unwrap();

    // appropriation
    let mut suspended_appropriation_table = TableLayout::new(vec![1,1]);
    let mut suspended_appropriation_row = suspended_appropriation_table.row();

    suspended_appropriation_row.push_element(Paragraph::new("Appropriation")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    suspended_appropriation_row.push_element(Paragraph::new(get_suspended_data.total_appropriation.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    suspended_appropriation_row.push().unwrap();

    // Contract Cost
    let mut suspended_contract_cost_table = TableLayout::new(vec![1,1]);
    let mut suspended_contract_cost_row = suspended_contract_cost_table.row();

    suspended_contract_cost_row.push_element(Paragraph::new("Contract Cost")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    suspended_contract_cost_row.push_element(Paragraph::new(get_suspended_data.total_contract_cost.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    suspended_contract_cost_row.push().unwrap();

    // Paid
    let mut suspended_paid_table = TableLayout::new(vec![1,1]);
    let mut suspended_paid_row = suspended_paid_table.row();

    suspended_paid_row.push_element(Paragraph::new("Paid")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    suspended_paid_row.push_element(Paragraph::new(get_suspended_data.total_paid.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    suspended_paid_row.push().unwrap();

    // Balance
    let mut suspended_balance_table = TableLayout::new(vec![1,1]);
    let mut suspended_balance_row = suspended_balance_table.row();

    suspended_balance_row.push_element(Paragraph::new("Balance")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    suspended_balance_row.push_element(Paragraph::new(get_suspended_data.total_balance.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    suspended_balance_row.push().unwrap();

    // Savings
    let mut suspended_savings_table = TableLayout::new(vec![1,1]);
    let mut suspended_savings_row = suspended_savings_table.row();

    suspended_savings_row.push_element(Paragraph::new("Savings")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    suspended_savings_row.push_element(Paragraph::new(get_suspended_data.total_savings.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    suspended_savings_row.push().unwrap();

    // Summary Title
    let summary_title = Paragraph::new("Summary")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))
        .padded(Margins::trbl(0, 0, 0, 5));

    // summary data
    let get_summary_data = get_parse_projects.data_summary;

    // projects
    let mut summary_no_projects_table = TableLayout::new(vec![1,1]);
    let mut summary_no_projects_row = summary_no_projects_table.row();

    summary_no_projects_row.push_element(Paragraph::new("No. Projects")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    summary_no_projects_row.push_element(Paragraph::new(get_summary_data.total_projects.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    summary_no_projects_row.push().unwrap();

    // appropriation
    let mut summary_appropriation_table = TableLayout::new(vec![1,1]);
    let mut summary_appropriation_row = summary_appropriation_table.row();

    summary_appropriation_row.push_element(Paragraph::new("Appropriation")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    summary_appropriation_row.push_element(Paragraph::new(get_summary_data.total_appropriation.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    summary_appropriation_row.push().unwrap();

    // Contract Cost
    let mut summary_contract_cost_table = TableLayout::new(vec![1,1]);
    let mut summary_contract_cost_row = summary_contract_cost_table.row();

    summary_contract_cost_row.push_element(Paragraph::new("Contract Cost")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    summary_contract_cost_row.push_element(Paragraph::new(get_summary_data.total_contract_cost.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    summary_contract_cost_row.push().unwrap();

    // Paid
    let mut summary_paid_table = TableLayout::new(vec![1,1]);
    let mut summary_paid_row = summary_paid_table.row();

    summary_paid_row.push_element(Paragraph::new("Paid")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    summary_paid_row.push_element(Paragraph::new(get_summary_data.total_paid.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    summary_paid_row.push().unwrap();

    // Balance
    let mut summary_balance_table = TableLayout::new(vec![1,1]);
    let mut summary_balance_row = summary_balance_table.row();

    summary_balance_row.push_element(Paragraph::new("Balance")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    summary_balance_row.push_element(Paragraph::new(get_summary_data.total_balance.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    summary_balance_row.push().unwrap();

    // Savings
    let mut summary_savings_table = TableLayout::new(vec![1,1]);
    let mut summary_savings_row = summary_savings_table.row();

    summary_savings_row.push_element(Paragraph::new("Savings")
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(8))
        .padded(Margins::trbl(0, 0, 0, 10)));
    summary_savings_row.push_element(Paragraph::new(get_summary_data.total_savings.to_formatted_string(&Locale::en))
        .aligned(Alignment::Left)
        .styled(style::Style::new().with_font_size(7)));

    summary_savings_row.push().unwrap();

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-financial-status-report", 2, "Compiling project summary informations");
    });

    doc.push(reports_title);
    doc.push(Break::new(1));
    doc.push(implemented_title);
    doc.push(Break::new(0.2));
    doc.push(implemented_no_projects_table);
    doc.push(Break::new(0.2));
    doc.push(implemented_appropriation_table);
    doc.push(Break::new(0.2));
    doc.push(implemented_contract_cost_table);
    doc.push(Break::new(0.2));
    doc.push(implemented_paid_table);
    doc.push(Break::new(0.2));
    doc.push(implemented_balance_table);
    doc.push(Break::new(0.2));
    doc.push(implemented_savings_table);

    doc.push(Break::new(1));
    doc.push(unimplemented_title);
    doc.push(Break::new(0.2));
    doc.push(unimplemented_no_projects_table);
    doc.push(Break::new(0.2));
    doc.push(unimplemented_appropriation_table);
    doc.push(Break::new(0.2));
    doc.push(unimplemented_contract_cost_table);
    doc.push(Break::new(0.2));
    doc.push(unimplemented_paid_table);
    doc.push(Break::new(0.2));
    doc.push(unimplemented_balance_table);
    doc.push(Break::new(0.2));
    doc.push(unimplemented_savings_table);

    doc.push(Break::new(1));
    doc.push(suspended_title);
    doc.push(Break::new(0.2));
    doc.push(suspended_no_projects_table);
    doc.push(Break::new(0.2));
    doc.push(suspended_appropriation_table);
    doc.push(Break::new(0.2));
    doc.push(suspended_contract_cost_table);
    doc.push(Break::new(0.2));
    doc.push(suspended_paid_table);
    doc.push(Break::new(0.2));
    doc.push(suspended_balance_table);
    doc.push(Break::new(0.2));
    doc.push(suspended_savings_table);

    doc.push(Break::new(1));
    doc.push(summary_title);
    doc.push(Break::new(0.2));
    doc.push(summary_no_projects_table);
    doc.push(Break::new(0.2));
    doc.push(summary_appropriation_table);
    doc.push(Break::new(0.2));
    doc.push(summary_contract_cost_table);
    doc.push(Break::new(0.2));
    doc.push(summary_paid_table);
    doc.push(Break::new(0.2));
    doc.push(summary_balance_table);
    doc.push(Break::new(0.2));
    doc.push(summary_savings_table);

    let mut buffer = Cursor::new(Vec::new());
    doc.render(&mut buffer).expect("Failed to render PDF Document");

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-financial-status-report", 3, "Generating PDF file");
    });

    let pdf_bytes = buffer.into_inner();

    let projects_file_name = format!("financial-status-reports.pdf");
    let attachment = format!("attachment; filename=\"{}\"", projects_file_name);

    // SSE EVENTS
    let clone_sse = sse.clone();
    tokio::spawn(async move {
        clone_sse.send_progress_update("projects-financial-status-report", 4, "PDF Report Completed");
    });

    Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, attachment)
        .header(header::CONTENT_LENGTH, pdf_bytes.len())
        .body(Body::from(pdf_bytes))
        .unwrap()
}