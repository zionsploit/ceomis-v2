use std::{io::Cursor, sync::Arc};

use axum::{body::Body, http::{header, Response}, response::IntoResponse, Extension, Json};
use entity::projects;
use genpdf::{elements::{Break, LinearLayout, Paragraph, TableLayout}, style, Alignment, Document, Element, Margins, Position};
use num_format::{Locale, ToFormattedString};
use sea_orm::EntityTrait;
use services::{db_connection::DB, redis::Redis, request::projects::RequestGenerateProjectsProfile, response::projects::ResponseViewProjectsById};

use crate::helpers::{app_icon_images, city_images, doc_title, font_family, left_text_paragraph};


pub async fn reports_projects_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestGenerateProjectsProfile>,
) -> impl IntoResponse {

    let sse = Arc::new(db.sse_connection.clone());    

    let get_find_project = projects::Entity::find_by_id(request.id)
        .one(&db.db_connection).await.unwrap();

    if get_find_project.is_none() {
        return Response::default();
    }

    let events_id = Arc::new(get_find_project.unwrap().project_code);

    let mut redis: Redis = Redis::new(format!("get_projects_by_id_{}", request.id), db.redis_connection.clone());

    let get_redis_value = redis.get_value().await;

    if get_redis_value.is_none() {
        let clone_sse = sse.clone();
        let clone_event = Arc::clone(&events_id);
        tokio::spawn(async move {
            clone_sse.send_progress_update(clone_event.as_str(), 0, "No Project Found");
        });

        return Response::default();
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 0, "Preparing project profile information");
    });

    let get_parse_projects: ResponseViewProjectsById = serde_json::from_str(&get_redis_value.unwrap()).unwrap();


    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 1, "Preparing report document");
    });

    
    // THE PDF GENERATING STARTS HERE
    // SAFELY THREADS,
    let mut doc = Document::new(font_family());
    doc.set_title("City Engineer's Office - Management Information System");

    doc.push(city_images().with_position(Position::new(40, 10)));
    doc.push(app_icon_images().with_position(Position::new(25, 10)));
    doc.push(doc_title());


    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 2, "Building infrastructure project profile");
    });

    // Project Information
    let reports_title = Paragraph::new("Infrastructure Project Profile")
        .aligned(Alignment::Center)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(17))
        .padded(Margins::trbl(5, 0, 0, 0));

    let projects_information = left_text_paragraph(
        Paragraph::new("Project Information")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))).padded(Margins::trbl(2, 0, 0, 0));


    // Project Name - Project Code
    let mut table_project_name_code = TableLayout::new(vec![1, 1]);
    let mut row_project_name_code = table_project_name_code.row();

    // Project Name
    let mut projects_name = LinearLayout::vertical();
    projects_name.push(
        left_text_paragraph(Paragraph::new("Project Name")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
        )
    );
    projects_name.push(
        left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.projects_name))
    ); 

    // Project Code
    let mut project_code = LinearLayout::vertical();
    project_code.push(
        left_text_paragraph(Paragraph::new("Project Code")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
        )
    );
    project_code.push(
        left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.projects_code.clone()))
    ); 

    row_project_name_code.push_element(projects_name);
    row_project_name_code.push_element(project_code);
    row_project_name_code.push().unwrap();

    // Project Appropriation - Project Type
    let mut table_project_appropriation_type = TableLayout::new(vec![1, 1]);
    let mut row_project_appropriation_type = table_project_appropriation_type.row();

    // Project Appropriation
    let mut project_appropriation = LinearLayout::vertical();
    project_appropriation.push(left_text_paragraph(Paragraph::new("Appropriation")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_appropriation.push(
        left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.projects_appropriation.unwrap_or(0).to_formatted_string(&Locale::en))
    ));

    // Project Type
    let mut project_type = LinearLayout::vertical();
    project_type.push(left_text_paragraph(Paragraph::new("Project type")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_type.push(
        left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.s_types_name.unwrap_or("None".to_owned()))
    ));

    row_project_appropriation_type.push_element(project_appropriation);
    row_project_appropriation_type.push_element(project_type);
    row_project_appropriation_type.push().unwrap();


    // Project Source Of funds - Sustainable Development Goals
    let mut table_project_source_of_funds_sustainable_development_goals = TableLayout::new(vec![1, 1]);
    let mut row_project_source_of_funds_sustainable_development_goals = table_project_source_of_funds_sustainable_development_goals.row();

    // Project Source of funds
    let mut project_source_of_funds = LinearLayout::vertical();
    project_source_of_funds.push(left_text_paragraph(Paragraph::new("Source of funds")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_source_of_funds.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.s_sof_name.unwrap_or("None".to_owned()))));

    // Project Sustainable Development Goals

    let sdg_data: Vec<String> = get_parse_projects.projects_info.projects_sdg.unwrap_or_default().iter().map(|v| v.name.to_owned()).collect();

    let mut project_sustainable_development_goals = LinearLayout::vertical();
    project_sustainable_development_goals.push(left_text_paragraph(Paragraph::new("Sustainable development goal")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));

    let sustainable_development_goals;

    if sdg_data.is_empty() {
        sustainable_development_goals = Paragraph::new("None".to_owned());
    } else {
        sustainable_development_goals = Paragraph::new(sdg_data.join(", "))
    }

    project_sustainable_development_goals.push(left_text_paragraph(sustainable_development_goals));

    row_project_source_of_funds_sustainable_development_goals.push_element(project_source_of_funds);
    row_project_source_of_funds_sustainable_development_goals.push_element(project_sustainable_development_goals);
    row_project_source_of_funds_sustainable_development_goals.push().unwrap();


    // Project Accomplishment - Status
    let mut table_project_accomplishment_status = TableLayout::new(vec![1, 1]);
    let mut row_project_accomplishment_status = table_project_accomplishment_status.row();

    // Project Accomplishment
    let mut project_accomplishment = LinearLayout::vertical();
    project_accomplishment.push(left_text_paragraph(Paragraph::new("% Accomplishment")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_accomplishment.push(left_text_paragraph(Paragraph::new(format!("{} %", get_parse_projects.projects_info.projects_accomplished.unwrap_or(0)))));

    // Project Status
    let mut project_status = LinearLayout::vertical();
    project_status.push(left_text_paragraph(Paragraph::new("Status")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_status.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.projects_status)));

    row_project_accomplishment_status.push_element(project_accomplishment);
    row_project_accomplishment_status.push_element(project_status);
    row_project_accomplishment_status.push().unwrap();


    // Project Barangay - Approved Budget Contract
    let mut table_project_barangay_approved_budget_contract = TableLayout::new(vec![1,1]);
    let mut row_project_barangay_approved_budget_contract = table_project_barangay_approved_budget_contract.row();

    // Project Barangay
    let barangay_data: Vec<String> = get_parse_projects.projects_info.projects_barangay.unwrap_or_default().iter().map(|v| v.name.to_owned()).collect();
    let mut project_barangay = LinearLayout::vertical();
    project_barangay.push(left_text_paragraph(Paragraph::new("Barangay")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));

    let barangay;
    if barangay_data.is_empty() {
        barangay = Paragraph::new("None".to_owned());
    } else {
        barangay = Paragraph::new(barangay_data.join(", "));
    }

    project_barangay.push(left_text_paragraph(barangay));

    // Project Approved Budget Contract
    let mut project_approved_budget_contract = LinearLayout::vertical();
    project_approved_budget_contract.push(left_text_paragraph(Paragraph::new("Approved Budget Contract")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_approved_budget_contract.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.projects_approved_budget_contract.unwrap_or(0).to_formatted_string(&Locale::en))));

    row_project_barangay_approved_budget_contract.push_element(project_barangay);
    row_project_barangay_approved_budget_contract.push_element(project_approved_budget_contract);
    row_project_barangay_approved_budget_contract.push().unwrap();


    // Project Category - Mode of Implementation
    let mut table_project_category_incharge = TableLayout::new(vec![1, 1]);
    let mut row_project_category_incharge = table_project_category_incharge.row();

    // Project Category
    let mut project_category = LinearLayout::vertical();
    project_category.push(left_text_paragraph(Paragraph::new("Category")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_category.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.s_category_name.unwrap_or("None".to_string()))));

    // Project Incharge
    let mut project_incharge = LinearLayout::vertical();
    project_incharge.push(left_text_paragraph(Paragraph::new("Mode of implementation")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_incharge.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.s_incharge_name.unwrap_or("None".to_string()))));

    row_project_category_incharge.push_element(project_category);
    row_project_category_incharge.push_element(project_incharge);
    row_project_category_incharge.push().unwrap();


    // Project Sector
    let sector_data: Vec<String> = get_parse_projects.projects_info.projects_sector.unwrap_or_default().iter().map(|v| v.name.to_owned()).collect();
    let mut project_sector = LinearLayout::vertical();
    project_sector.push(left_text_paragraph(Paragraph::new("Sector")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    
    let sector;
    if sector_data.is_empty() {
        sector = Paragraph::new("None".to_owned());
    } else {
        sector = Paragraph::new(sector_data.join(", "));
    }

    project_sector.push(left_text_paragraph(sector));


    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 3, "Building project contract details");
    });

    // Project contract details
    let project_contract_details = left_text_paragraph(
        Paragraph::new("Project Contract Details")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10)));

    
    // Project Contractor - Contract Cost
    let mut table_project_contract_contract_cost = TableLayout::new(vec![1,1]);
    let mut row_project_contract_contract_cost = table_project_contract_contract_cost.row();

    // Project Contractor
    let mut project_contractor = LinearLayout::vertical();
    project_contractor.push(left_text_paragraph(Paragraph::new("Contractor")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_contractor.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.contractor_name.unwrap_or("None".to_string()))));


    // Project Contract Cost
    let mut project_contract_cost = LinearLayout::vertical();
    project_contract_cost.push(left_text_paragraph(Paragraph::new("Contract cost")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_contract_cost.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.projects_contract_cost.unwrap_or(0).to_formatted_string(&Locale::en))));

    row_project_contract_contract_cost.push_element(project_contractor);
    row_project_contract_contract_cost.push_element(project_contract_cost);
    row_project_contract_contract_cost.push().unwrap();


    // Project Start Date - Target Date
    let mut table_project_start_date_target_date = TableLayout::new(vec![1,1]);
    let mut row_project_start_date_target_date = table_project_start_date_target_date.row();

    // Project Start Date
    let mut project_start_date = LinearLayout::vertical();
    project_start_date.push(left_text_paragraph(Paragraph::new("Start Date")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_start_date.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.projects_start_date.unwrap_or("None".to_string()))));

    // Project Target Date
    let mut project_target_data = LinearLayout::vertical();
    project_target_data.push(left_text_paragraph(Paragraph::new("Target Date")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_target_data.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.projects_target_date.clone().unwrap_or("None".to_string()))));


    row_project_start_date_target_date.push_element(project_start_date);
    row_project_start_date_target_date.push_element(project_target_data);
    row_project_start_date_target_date.push().unwrap();


    // Project Taker - Actual Date Completed
    let mut table_project_taker_actual_date_completed = TableLayout::new(vec![1,1]);
    let mut row_project_taker_actual_date_completed = table_project_taker_actual_date_completed.row();

    // Project Taker
    let mut project_taker = LinearLayout::vertical();
    project_taker.push(left_text_paragraph(Paragraph::new("Taker")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_taker.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.s_takers_name.unwrap_or("None".to_string()))));

    // Project Actual Date Completed
    let mut project_actual_date_completed = LinearLayout::vertical();
    project_actual_date_completed.push(left_text_paragraph(Paragraph::new("Actual Date Completed")
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
    ));
    project_actual_date_completed.push(left_text_paragraph(Paragraph::new(get_parse_projects.projects_info.projects_target_date.unwrap_or("None".to_string()))));


    row_project_taker_actual_date_completed.push_element(project_taker);
    row_project_taker_actual_date_completed.push_element(project_actual_date_completed);
    row_project_taker_actual_date_completed.push().unwrap();


    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 4, "Building project course of action");
    });
    
    // Project Course Of Action
    let project_course_of_action = left_text_paragraph(
        Paragraph::new("Course of action")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10)));


    // Course Of Action Table
    let mut table_course_of_action_table = TableLayout::new(vec![1,1,1,1]);
    
    // ROW HEAD
    {
        let mut row_course_of_action_table_head = table_course_of_action_table.row();

        row_course_of_action_table_head.push_element(left_text_paragraph(
        Paragraph::new("Status")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))));

        row_course_of_action_table_head.push_element(left_text_paragraph(
            Paragraph::new("Processed by")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))));

        row_course_of_action_table_head.push_element(left_text_paragraph(
            Paragraph::new("Date & time")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))));

        row_course_of_action_table_head.push_element(left_text_paragraph(
            Paragraph::new("Remarks")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))));

        row_course_of_action_table_head.push().unwrap();
    }

    let course_of_action_is_none = Paragraph::new("None").aligned(Alignment::Center);
    // BASIC MOCK DATA
    // {
    //     let mut row_course_of_action_table_body = table_course_of_action_table.row();
        
    //     row_course_of_action_table_body.push_element(left_text_paragraph(Paragraph::new("Data 1")));
    //     row_course_of_action_table_body.push_element(left_text_paragraph(Paragraph::new("Data 2")));
    //     row_course_of_action_table_body.push_element(left_text_paragraph(Paragraph::new("Data 3")));
    //     row_course_of_action_table_body.push_element(left_text_paragraph(Paragraph::new("Data 4")));

    //     row_course_of_action_table_body.push().unwrap();
    // }


    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 5, "Compiling project payment details");
    });

    // Project Payment
    let project_payments = left_text_paragraph(
        Paragraph::new("Payment")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10)));
    
    
    // Payment Table
    let mut table_payment_table = TableLayout::new(vec![3,2,2,2,2,2]);
    
    // ROW HEAD
    {
        let mut row_table_payment_table_head = table_payment_table.row();

        row_table_payment_table_head.push_element(Paragraph::new("Billing Date")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10)).padded(Margins::trbl(0, 0, 0, 10)));

        row_table_payment_table_head.push_element(Paragraph::new("Processed by")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10)));

        row_table_payment_table_head.push_element(Paragraph::new("Amount Due")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10)));

        row_table_payment_table_head.push_element(Paragraph::new("Amount Paid")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10)));

        row_table_payment_table_head.push_element(Paragraph::new("Reference")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10)));

        row_table_payment_table_head.push_element(Paragraph::new("Date of Payment")
            .aligned(Alignment::Left)
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10)));

        row_table_payment_table_head.push().unwrap();
    }

    for project_payment in get_parse_projects.projects_payment.clone()  {
        let mut row_table_payment_data = table_payment_table.row();

        row_table_payment_data.push_element(Paragraph::new(project_payment.billing_date).padded(Margins::trbl(0, 0, 0, 10)));
        row_table_payment_data.push_element(Paragraph::new(project_payment.processed_by));
        row_table_payment_data.push_element(Paragraph::new(project_payment.amount_due));
        row_table_payment_data.push_element(Paragraph::new(project_payment.amount_paid.to_formatted_string(&Locale::en)));
        row_table_payment_data.push_element(Paragraph::new(project_payment.reference_no));
        row_table_payment_data.push_element(Paragraph::new(project_payment.payment_date));

        row_table_payment_data.push().unwrap();
    }

    let no_project_payment = Paragraph::new("None").aligned(Alignment::Center);
    
    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 6, "Finalizing documents");
    });

    // INSERT INTO DOCS
    doc.push(reports_title);
    doc.push(projects_information);
    doc.push(Break::new(0.5));
    doc.push(table_project_name_code);
    doc.push(Break::new(0.5));
    doc.push(table_project_appropriation_type);
    doc.push(Break::new(0.5));
    doc.push(table_project_source_of_funds_sustainable_development_goals);
    doc.push(Break::new(0.5));
    doc.push(table_project_accomplishment_status);
    doc.push(Break::new(0.5));
    doc.push(table_project_barangay_approved_budget_contract);
    doc.push(Break::new(0.5));
    doc.push(table_project_category_incharge);
    doc.push(Break::new(0.5));
    doc.push(project_sector);
    doc.push(Break::new(1));
    doc.push(project_contract_details);
    doc.push(Break::new(0.5));
    doc.push(table_project_contract_contract_cost);
    doc.push(Break::new(0.5));
    doc.push(table_project_start_date_target_date);
    doc.push(Break::new(0.5));
    doc.push(table_project_taker_actual_date_completed);
    doc.push(Break::new(1));
    doc.push(project_course_of_action);
    doc.push(Break::new(0.5));
    doc.push(table_course_of_action_table);
    doc.push(course_of_action_is_none);
    doc.push(Break::new(1));
    doc.push(project_payments);
    doc.push(Break::new(0.5));
    doc.push(table_payment_table);

    if get_parse_projects.projects_payment.is_empty() {
        doc.push(no_project_payment);
    }
    
    doc.push(Break::new(1));

    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 7, "Generating PDF file");
    });

    let mut buffer = Cursor::new(Vec::new());
    doc.render(&mut buffer).expect("Failed to render PDF Document");

    let pdf_bytes = buffer.into_inner();


    let projects_file_name = format!("{}.pdf", get_parse_projects.projects_info.projects_code);
    let attachment = format!("attachment; filename=\"{}\"", projects_file_name);

    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 8, "PDF report completed");
    });

    Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, attachment)
        .header(header::CONTENT_LENGTH, pdf_bytes.len())
        .body(Body::from(pdf_bytes))
        .unwrap()

}