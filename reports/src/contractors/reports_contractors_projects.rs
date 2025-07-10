use std::{io::Cursor, sync::Arc};

use axum::{body::Body, http::{header, Response}, response::IntoResponse, Extension, Json};
use entity::contractors;
use genpdf::{elements::{Break, LinearLayout, Paragraph, TableLayout}, style, Alignment, Document, Element, Margins, PaperSize, Position};
use num_format::{Locale, ToFormattedString};
use sea_orm::EntityTrait;
use services::{db_connection::DB, redis::Redis, request::contractors::RequestGenerateProjectsContractors, response::contractors::ResponseContractorsWithProjects};

use crate::helpers::{app_icon_images, city_images, doc_title, font_family, left_text_paragraph};



#[axum::debug_handler]
pub async fn reports_contractors_projects_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestGenerateProjectsContractors>,
) -> impl IntoResponse {
    let sse = Arc::new(db.sse_connection.clone());    

    let get_find_contractors = contractors::Entity::find_by_id(request.id)
        .one(&db.db_connection).await.unwrap();

    if get_find_contractors.is_none() {
        return Response::default();
    }

    let events_id = Arc::new(get_find_contractors.unwrap().name);
    
    let mut redis = Redis::new(format!("get_contractors_by_id_{}", request.id), db.redis_connection.clone());

    let get_redis_value = redis.get_value().await;

    if get_redis_value.is_none() {
        // SSE EVENTS
        let clone_sse = sse.clone();
        let clone_event = Arc::clone(&events_id);
        tokio::spawn(async move {
            clone_sse.send_progress_update(clone_event.as_str(), 0, "No Contractors Found");
        });

        return Response::default();
    }

        // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 0, "Preparing contractors profile information");
    });

    let get_parse_contractors: ResponseContractorsWithProjects = serde_json::from_str(&get_redis_value.unwrap()).unwrap();


    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 1, "Preparing report document");
    });


    // Response
    // THE PDF GENERATING STARTS HERE
    // SAFELY THREADS,
    let mut doc = Document::new(font_family());
    doc.set_title("City Engineer's Office - Management Information System");
    doc.set_paper_size(PaperSize::Legal);

    doc.push(city_images().with_position(Position::new(40, 10)));
    doc.push(app_icon_images().with_position(Position::new(25, 10)));
    doc.push(doc_title());

    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 2, "Building contractors information");
    });

    // Project Information
    let reports_title = Paragraph::new("Infrastructure Contractor Profile")
        .aligned(Alignment::Center)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(17))
        .padded(Margins::trbl(5, 0, 0, 0));

    let projects_information = left_text_paragraph(
        Paragraph::new("Contractor Information")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))).padded(Margins::trbl(2, 0, 0, 0));

    

    
    // Contractors & State Address
    let mut table_contractors = TableLayout::new(vec![1,1]);
    let mut row_contractors = table_contractors.row();
    
    // Contractors Name
    let mut contractors_layout = LinearLayout::vertical();
    contractors_layout.push(left_text_paragraph(Paragraph::new("Contractor's Name")
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))));
    contractors_layout.push(left_text_paragraph(Paragraph::new(&get_parse_contractors.contractor.name)));
    
    // State Address
    let mut address_state_layout = LinearLayout::vertical();
    address_state_layout.push(left_text_paragraph(Paragraph::new("State / Province")
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))));
    address_state_layout.push(left_text_paragraph(Paragraph::new(&get_parse_contractors.contractor.address_province)));

    row_contractors.push_element(contractors_layout);
    row_contractors.push_element(address_state_layout);
    row_contractors.push().unwrap();


    // City & Barangay Address
    let mut table_city_barangay_address = TableLayout::new(vec![1,1]);
    let mut row_city_barangay_address = table_city_barangay_address.row();

    // City
    let mut city_layout = LinearLayout::vertical();
    city_layout.push(left_text_paragraph(Paragraph::new("City / Municipality")
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))));
    city_layout.push(left_text_paragraph(Paragraph::new(&get_parse_contractors.contractor.address_municipality)));

    
    // Barangay
    let mut barangay_layout = LinearLayout::vertical();
    barangay_layout.push(left_text_paragraph(Paragraph::new("Barangay")
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))));
    barangay_layout.push(left_text_paragraph(Paragraph::new(&get_parse_contractors.contractor.address_barangay)));

    row_city_barangay_address.push_element(city_layout);
    row_city_barangay_address.push_element(barangay_layout);
    row_city_barangay_address.push().unwrap();

    // Address Street
    let mut street_layout = LinearLayout::vertical();
    street_layout.push(left_text_paragraph(Paragraph::new("Street / Purok")
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))));
    street_layout.push(left_text_paragraph(Paragraph::new(&get_parse_contractors.contractor.address_street)));


    // Contact Information
    let contact_information = left_text_paragraph(
        Paragraph::new("Contact Information")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))).padded(Margins::trbl(2, 0, 0, 0));


    // Contact Person & Position
    let mut table_contact_person_position = TableLayout::new(vec![1,1]);
    let mut row_contact_person_position = table_contact_person_position.row();

    // Contact Person
    let mut contact_person_layout = LinearLayout::vertical();
    contact_person_layout.push(left_text_paragraph(Paragraph::new("Contact Person")
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))));
    contact_person_layout.push(left_text_paragraph(Paragraph::new(&get_parse_contractors.contractor.contact_full_name)));

    // Position
    let mut position_layout = LinearLayout::vertical();
    position_layout.push(left_text_paragraph(Paragraph::new("Position")
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))));
    position_layout.push(left_text_paragraph(Paragraph::new(&get_parse_contractors.contractor.contact_position)));

    row_contact_person_position.push_element(contact_person_layout);
    row_contact_person_position.push_element(position_layout);
    row_contact_person_position.push().unwrap();

    // Contact No. & Email
    let mut table_contact_no_email = TableLayout::new(vec![1,1]);
    let mut row_contact_no_email = table_contact_no_email.row();

    // Contact No
    let mut contact_no_layout = LinearLayout::vertical();
    contact_no_layout.push(left_text_paragraph(Paragraph::new("Contact No")
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))));
    contact_no_layout.push(left_text_paragraph(Paragraph::new(&get_parse_contractors.contractor.contact_number)));

    // Email
    let mut email_layout = LinearLayout::vertical();
    email_layout.push(left_text_paragraph(Paragraph::new("Email")
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))));
    email_layout.push(left_text_paragraph(Paragraph::new(&get_parse_contractors.contractor.email_address)));

    row_contact_no_email.push_element(contact_no_layout);
    row_contact_no_email.push_element(email_layout);
    row_contact_no_email.push().unwrap();


     // Contact Information
    let awarded_projects = left_text_paragraph(
        Paragraph::new("Awarded Projects")
        .aligned(Alignment::Left)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(10))).padded(Margins::trbl(2, 0, 0, 0));


    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 3, "Building awarded projects information");
    });

    // Awarded Projects Table
    let mut table_awarded_projects = TableLayout::new(vec![6,3,3,2,2,2]);
    // TABLE HEADED
    {
        let mut row_awarded_projects = table_awarded_projects.row();
        
        row_awarded_projects.push_element(Paragraph::new("Name").padded(Margins::trbl(0, 0, 0, 10)).styled(style::Style::new().with_font_size(7)).styled(style::Effect::Bold));
        row_awarded_projects.push_element(Paragraph::new("Code").padded(Margins::trbl(0, 0, 0, 5)).styled(style::Style::new().with_font_size(7)).styled(style::Effect::Bold));
        row_awarded_projects.push_element(Paragraph::new("Status").styled(style::Style::new().with_font_size(7)).styled(style::Effect::Bold));
        row_awarded_projects.push_element(Paragraph::new("Taker").styled(style::Style::new().with_font_size(7)).styled(style::Effect::Bold));
        row_awarded_projects.push_element(Paragraph::new("Appropriation").styled(style::Style::new().with_font_size(7)).styled(style::Effect::Bold));
        row_awarded_projects.push_element(Paragraph::new("Contract Cost").styled(style::Style::new().with_font_size(7)).styled(style::Effect::Bold));

        row_awarded_projects.push().unwrap();
    }

    for project in get_parse_contractors.projects {
        let mut row_awarded_projects = table_awarded_projects.row();

        row_awarded_projects.push_element(Paragraph::new(project.project_name).padded(Margins::trbl(0, 0, 0, 10)).styled(style::Style::new().with_font_size(7)));
        row_awarded_projects.push_element(Paragraph::new(project.project_code).padded(Margins::trbl(0, 0, 0, 5)).styled(style::Style::new().with_font_size(7)));
        row_awarded_projects.push_element(Paragraph::new(project.project_status.unwrap_or("".to_string())).styled(style::Style::new().with_font_size(7)));
        row_awarded_projects.push_element(Paragraph::new(project.project_takers.unwrap_or("".to_string())).styled(style::Style::new().with_font_size(7)));
        row_awarded_projects.push_element(Paragraph::new(project.project_appropriation.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));
        row_awarded_projects.push_element(Paragraph::new(project.project_contract_cost.to_formatted_string(&Locale::en)).styled(style::Style::new().with_font_size(7)));

        row_awarded_projects.push().unwrap();

        let mut row_awarded_projects = table_awarded_projects.row();

        for _n in [1; 6] {
            row_awarded_projects.push_element(Break::new(0.5));
        }
        row_awarded_projects.push().unwrap();

    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 4, "Compiling contractors informations");
    });

    // INSERT INTO DOCS
    doc.push(reports_title);
    doc.push(Break::new(1));
    doc.push(projects_information);
    doc.push(Break::new(0.5));
    doc.push(table_contractors);
    doc.push(Break::new(0.5));
    doc.push(table_city_barangay_address);
    doc.push(Break::new(0.5));
    doc.push(street_layout);
    doc.push(Break::new(1));
    doc.push(contact_information);
    doc.push(Break::new(0.5));
    doc.push(table_contact_person_position);
    doc.push(Break::new(0.5));
    doc.push(table_contact_no_email);
    doc.push(Break::new(1));
    doc.push(awarded_projects);
    doc.push(Break::new(0.5));
    doc.push(table_awarded_projects);
    doc.push(Break::new(0.5));


    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 5, "Generating PDF file");
    });

    let mut buffer = Cursor::new(Vec::new());
    doc.render(&mut buffer).expect("Failed to render PDF Document");

    let pdf_bytes = buffer.into_inner();


    let projects_file_name = format!("{}.pdf", get_parse_contractors.contractor.name);
    let attachment = format!("attachment; filename=\"{}\"", projects_file_name);

    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 6, "PDF Report Completed");
    });

    Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, attachment)
        .header(header::CONTENT_LENGTH, pdf_bytes.len())
        .body(Body::from(pdf_bytes))
        .unwrap()
}