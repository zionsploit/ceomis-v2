use std::{collections::HashMap, sync::Arc};

use aws_sdk_s3::primitives::ByteStream;
use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Extension};
use entity::{projects_monitoring_img, projects_monitoring_remarks};
use sea_orm::ActiveValue::Set;
use sea_orm::ActiveModelTrait;
use services::db_connection::DB;
use services::redis::Redis;
use tracing::error;

struct ImagesHash {
    data: ByteStream,
    key: String,
    content_type: String,
    original_name: String,
}

pub async fn add_projects_remarks(
    Extension(db): Extension<Arc<DB>>,
    mut multipart: Multipart
) -> impl IntoResponse {

    let mut images_hash: HashMap<String, ImagesHash> = HashMap::new();
    let mut remarks_data: HashMap<String, String> = HashMap::new();
    
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(content_type) = field.content_type() {
            let file_content_type = content_type.to_string();
            
            if let Ok(mime_type) = file_content_type.parse::<mime::Mime>() {
                if mime_type.type_() == mime::IMAGE {

                    let content_type = field.content_type().unwrap().to_string();
                    let file_name = field.file_name().unwrap_or("unknown").to_string();
                    let bytes = field.bytes().await.unwrap();
                    let data = ByteStream::from(bytes);

                    let key = uuid::Uuid::new_v4().to_string();

                    images_hash.insert(key.to_owned(), ImagesHash { 
                        data: data, 
                        key: key, 
                        content_type: content_type, 
                        original_name: file_name, 
                    });
                }
            }
        }
        else {
            // This is likely a text field
            let name = field.name().unwrap_or("unknown").to_string();
            let text_data = field.text().await.unwrap();
            remarks_data.insert(name.to_owned(), text_data);            
        }
    }

    let get_remarks_project_id = remarks_data.get("project_id");
    let get_remarks_text = remarks_data.get("remarks");
    let get_remarks_date = remarks_data.get("remarks_date");

    if get_remarks_project_id.is_none() {
        return (StatusCode::UNPROCESSABLE_ENTITY, format!("Project id entity Not found"));
    }

    if get_remarks_text.is_none() {
        return (StatusCode::UNPROCESSABLE_ENTITY, format!("Remarks entity Not found"));
    }

    if get_remarks_date.is_none() {
        return (StatusCode::UNPROCESSABLE_ENTITY, format!("Remarks date entity Not found"));
    }

    let response_remarks_id  = {
        let make_add_remarks = projects_monitoring_remarks::ActiveModel {
            remarks: Set(get_remarks_text.unwrap().to_string()),
            remarks_date: Set(get_remarks_date.unwrap().to_string()),
            project_id: Set(get_remarks_project_id.clone().unwrap().parse().unwrap()),
            ..Default::default()
        };

        let response_add_remarks = make_add_remarks.save(&db.db_connection).await.unwrap();
        
        String::from(format!("{}", response_add_remarks.id.into_value().unwrap())).parse().unwrap()
    };


    for images in images_hash {
        let (_name, data) = images;

        let s3 = db.s3_connection.clone();

        let object_key = data.key;

        let s3_results = s3.put_object()
            .bucket("projects-images")
            .body(data.data)
            .key(&object_key)
            .content_type(data.content_type)
            .send()
            .await;

        match s3_results {
            Ok(_) => {
                let make_add_remarks_img = projects_monitoring_img::ActiveModel {
                    images_key: Set(object_key),
                    images_original_name: Set(data.original_name),
                    remarks_id: Set(response_remarks_id),
                    ..Default::default()
                };

                if let Err(err) = make_add_remarks_img.save(&db.db_connection).await {
                    error!("{}", err.sql_err().unwrap())
                }
            },
            Err(err) => error!("{:?}", err)
        }
    }

    Redis::new(format!("get_projects_by_id_{}", get_remarks_project_id.unwrap()), db.redis_connection.clone())
        .remove_value_if_exists().await.unwrap();
    
    (StatusCode::CREATED, format!("{}", ""))
}
