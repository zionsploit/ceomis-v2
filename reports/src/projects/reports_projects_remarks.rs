use std::{collections::{HashMap, HashSet}, io::Cursor, sync::Arc};

use axum::{body::Body, http::{header, Response}, response::IntoResponse, Extension, Json};
use entity::projects;
use genpdf::{elements::{Break, Image, LinearLayout, Paragraph, TableLayout}, style, Alignment, Document, Element, Margins, Position};
use sea_orm::EntityTrait;
use services::{db_connection::DB, redis::Redis, request::projects::RequestGenerateProjectsProfile, response::projects::{ResponseProjectRemarks, ResponseProjectRemarksImg, ResponseViewProjectsById}};

use crate::helpers::{app_icon_images, city_images, convert_img_into_jpeg, doc_title, font_family};


struct ProjectRemarksData {
    remarks: String,
    remarks_data: String,
    remarks_img: HashSet<Vec<u8>>
}

struct ProjectRemarksImg {
    img_bytes: Vec<u8>,
    remarks_img_data: ResponseProjectRemarksImg
}

pub async fn reports_projects_remarks_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestGenerateProjectsProfile>
) -> impl IntoResponse {

    // let s3 = db.s3_connection.clone();
    // let bucket = s3.get_object().bucket("projects-images");
    
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
        // SSE EVENTS
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


    let get_project_remarks: Vec<ResponseProjectRemarks> = get_parse_projects.projects_remarks;
    let get_project_remarks_img: Vec<ResponseProjectRemarksImg> = get_parse_projects.projects_remarks_image;

    let mut project_remarks_data: HashMap<i32, ProjectRemarksData> = HashMap::new();

    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 1, "Retrieving project remarks data and images");
    });


    let mut remarks_img_data: HashMap<usize, ProjectRemarksImg> = HashMap::new(); 

    let get_project_remarks_img_total = get_project_remarks_img.len();

    for (n, project_remark_img) in get_project_remarks_img.iter().enumerate() {

        // remarks_img_data
            // SSE EVENTS
        let clone_sse = sse.clone();
        let clone_event = Arc::clone(&events_id);
        tokio::spawn(async move {
            clone_sse.send_progress_update(clone_event.as_str(), 100, format!("{} of {} images", n + 1, get_project_remarks_img_total).as_str());
        });

        // let get_objects = bucket.clone()
        //     .key(project_remark_img.images_key.clone())
        //     .send().await.unwrap();
        // let object_bytes = get_objects.body
        //     .collect().await.unwrap().clone().into_bytes();
        // let data_img = convert_img_into_jpeg(object_bytes);

        // let clone_project_remark_img = project_remark_img.clone().to_owned();

        // remarks_img_data.insert(n, ProjectRemarksImg { img_bytes: data_img, remarks_img_data: clone_project_remark_img.to_owned().to_owned() });

    }

    for project_remarks in get_project_remarks {
        let get_project_remark_img = remarks_img_data.iter().filter_map(|(_n , v)| {
            if v.remarks_img_data.remarks_id == project_remarks.id {
                Some(v.img_bytes.clone())
            } else {
                None
            }
        }).collect::<HashSet<Vec<u8>>>();

        project_remarks_data.insert(project_remarks.id, ProjectRemarksData { 
            remarks: project_remarks.remarks, 
            remarks_data: project_remarks.remarks_date,
            remarks_img: get_project_remark_img 
        });
    }


    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 2, "Preparing report document");
    });

    let mut doc = Document::new(font_family());
    doc.set_title("City Engineer's Office - Management Information System");

    doc.push(city_images().with_position(Position::new(40, 10)));
    doc.push(app_icon_images().with_position(Position::new(25, 10)));
    doc.push(doc_title());


    // Project Title
    let mut reports_title = LinearLayout::vertical();
    reports_title.push(Paragraph::new("Infrastructure Project Profile")
        .aligned(Alignment::Center)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(17))
        .padded(Margins::trbl(5, 0, 0, 0)));
    reports_title.push(Paragraph::new("Project Remarks")
        .aligned(Alignment::Center)
        .styled(style::Effect::Bold)
        .styled(style::Style::new().with_font_size(12)));

    doc.push(reports_title); 


    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 3, "Building project remarks details");
    });

    for remarks_data in project_remarks_data {
        let (_, projects_data) = remarks_data;

        let remarks_img: Vec<Vec<u8>> = projects_data.remarks_img.iter().cloned().collect();
        let chunks_img: Vec<Vec<Vec<u8>>> = remarks_img.chunks(4).map(|chunk| chunk.to_vec()).collect();

        doc.push(Paragraph::new(projects_data.remarks.clone())
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(14))
            .padded(Margins::trbl(0, 0, 0, 10)));
        doc.push(Paragraph::new(projects_data.remarks_data.clone())
            .styled(style::Effect::Bold)
            .styled(style::Style::new().with_font_size(10))
            .padded(Margins::trbl(0, 0, 0, 10)));

        // Iterate image chunks (in sets of 3)
        for chunk in chunks_img {
            let mut table = TableLayout::new(vec![2; chunk.len()]);
            let mut row = table.row();

            for img_bytes in chunk {
                let image_element = Image::from_reader(Cursor::new(img_bytes)).unwrap();
                row.push_element(image_element);
            }

            row.push().unwrap();

            // Push table to layout
            doc.push(table.padded(Margins::trbl(5, 0, 0, 10)));

            // 🔸 Insert a manual soft page break to avoid clipping if too much content
            doc.push(Break::new(1));
        }
    }

    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 4, "Generating PDF file");
    });

    let mut buffer = Cursor::new(Vec::new());
    doc.render(&mut buffer).expect("Failed to render PDF Document");

    let pdf_bytes = buffer.into_inner();

    let projects_file_name = format!("remarks_{}.pdf", get_parse_projects.projects_info.projects_code);
    let attachment = format!("attachment; filename=\"{}\"", projects_file_name);

    // SSE EVENTS
    let clone_sse = sse.clone();
    let clone_event = Arc::clone(&events_id);
    tokio::spawn(async move {
        clone_sse.send_progress_update(clone_event.as_str(), 5, "PDF report completed");
    });

    Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, attachment)
        .header(header::CONTENT_LENGTH, pdf_bytes.len())
        .body(Body::from(pdf_bytes))
        .unwrap()
}