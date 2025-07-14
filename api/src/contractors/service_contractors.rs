use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{contractors, projects, settings::settings_takers, user};
use sea_orm::{prelude::*, ActiveValue::{NotSet, Set}, Condition, IntoActiveModel, QueryOrder, QuerySelect, SelectColumns};
use services::{db_connection::DB, redis::Redis, request::contractors::{RequestAddContractors, RequestDeleteContractorsById, RequestUpdateContractors}, response::{contractors::{ResponseContractors, ResponseContractorsWithProjects}, projects::ResponseProjectForContractors}};


// GET REQUEST
pub async fn get_all_contractors (
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_all_contractors".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = contractors::Entity::find()
        .filter(contractors::Column::IsDelete.eq(false))
        .order_by_asc(contractors::Column::Id).all(&db.db_connection).await.unwrap();

    redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(response))
}

pub async fn get_contractors_by_id (
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {

    let mut redis = Redis::new(format!("get_contractors_by_id_{}", id), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let find_contractors = contractors::Entity::find_by_id(id)
        .filter(contractors::Column::IsDelete.eq(false)).one(&db.db_connection).await.unwrap();

    if find_contractors.is_none() {
        return (StatusCode::NOT_FOUND, Json::default());
    }

    // If you want to load the related project, you need to use LoaderTrait on a slice or manually query it.
    let contractor = find_contractors.clone().unwrap();

    let find_projects = projects::Entity::find()
        .filter(Condition::all()
            .add(projects::Column::ContractorId.eq(contractor.id))
            .add(projects::Column::IsDisposed.eq(false)))
        .left_join(settings_takers::Entity)
        .select_only()
        .select_column_as(projects::Column::Id, "project_id")
        .select_column_as(projects::Column::ProjectName, "project_name")
        .select_column_as(projects::Column::ProjectCode, "project_code")
        .select_column_as(projects::Column::ProjectStatus, "project_status")
        .select_column_as(settings_takers::Column::Name, "project_takers")
        .select_column_as(projects::Column::Appropriation, "project_appropriation")
        .select_column_as(projects::Column::ContractCost, "project_contract_cost")
        .into_model::<ResponseProjectForContractors>()
        .all(&db.db_connection)
        .await
        .unwrap();


    let parse_contractor = ResponseContractors {
        id: contractor.id,
        name: contractor.name,
        email_address: contractor.email_address,
        address_street: contractor.address_street,
        address_barangay: contractor.address_barangay,
        address_municipality: contractor.address_municipality,
        address_province: contractor.address_province,
        about: contractor.about,
        contact_full_name: contractor.contact_full_name,
        contact_position: contractor.contact_position,
        contact_number: contractor.contact_number,
        is_delete: contractor.is_delete,
    };

    let make_response = ResponseContractorsWithProjects {
        contractor: parse_contractor,
        projects: find_projects,
    };

    redis.stored_value(&serde_json::to_string(&make_response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(make_response))

}


// POST REQUEST
pub async fn add_contractors(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddContractors>
) -> impl IntoResponse {

    let make_add = contractors::ActiveModel {
        name: Set(request.name),
        email_address: Set(request.email_address),
        address_street: Set(request.address_street),
        address_barangay: Set(request.address_barangay),
        address_municipality: Set(request.address_municipality),
        address_province: Set(request.address_province),
        about: match request.about {
            Some(res) => Set(Some(res)),
            None => NotSet
        },
        contact_full_name: Set(request.contact_full_name),
        contact_position: Set(request.contact_position),
        contact_number: Set(request.contact_number),
        ..Default::default()
    };

    let response = make_add.save(&db.db_connection).await.unwrap();

    Redis::new("get_all_contractors".to_string(), db.redis_connection.clone())
        .remove_value().await.unwrap();

    (StatusCode::CREATED, format!("{}", response.id.into_value().unwrap()))
}

pub async fn update_contractors(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpdateContractors>
) -> impl IntoResponse {

    let find_contractors = contractors::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if find_contractors.is_none() {
        return (StatusCode::NOT_FOUND, format!(""));
    }

    let mut make_update = find_contractors.unwrap().into_active_model();

    make_update.name = Set(request.name);
    make_update.email_address = Set(request.email_address);
    make_update.address_street = Set(request.address_street);
    make_update.address_barangay = Set(request.address_barangay);
    make_update.address_municipality = Set(request.address_municipality);
    make_update.address_province = Set(request.address_province);
    make_update.about = match request.about {
        Some(req) => Set(Some(req)),
        None => NotSet
    };
    make_update.contact_full_name = Set(request.contact_full_name);
    make_update.contact_position = Set(request.contact_position);
    make_update.contact_number = Set(request.contact_number);

    let response = make_update.update(&db.db_connection).await.unwrap();

    Redis::new("get_all_contractors".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![
            &format!("get_contractors_by_id_{}", request.id)
        ]).await.unwrap();

    (StatusCode::CREATED, format!("{}", response.id))
}

pub async fn delete_contractors_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteContractorsById>
) -> impl IntoResponse {
    
    let find_hashed_users = user::Entity::find()
        .filter(user::Column::Password.eq(request.hash_password())).one(&db.db_connection).await.unwrap();

    if find_hashed_users.is_none() {
        return (StatusCode::UNPROCESSABLE_ENTITY, format!(""));
    }

    let response = contractors::Entity::update(contractors::ActiveModel {
        id: Set(request.id),
        is_delete: Set(true),
        ..Default::default()
    }).exec(&db.db_connection).await.unwrap();

    Redis::new("get_all_contractors".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![
            &format!("get_contractors_by_id_{}", response.id)
        ]).await.unwrap();

    (StatusCode::CREATED, format!("{}", response.id))
}
