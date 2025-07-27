use std::{collections::{HashMap, HashSet}, sync::Arc};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{contractors, projects::{self, ProjectStatus}, projects_infra_code, projects_monitoring_img, projects_monitoring_remarks, projects_payment, settings::{settings_barangay, settings_categories, settings_incharge, settings_sdg, settings_sector, settings_sof, settings_takers, settings_type}, user};
use sea_orm::{prelude::*, sea_query::{ExprTrait, Func}, ActiveValue::{NotSet, Set}, Condition, EntityTrait, FromQueryResult, IntoActiveModel, QueryFilter, QueryOrder, QuerySelect, SelectColumns};
use serde::{Deserialize, Serialize};
use services::{db_connection::DB, redis::Redis, request::projects::{RequestAddProjects, RequestDeleteProjectRemarks, RequestDisposedProjectsById, RequestUpdateProjects, RequestUpsertProjectsInfraCode}, response::{projects::{ResponseProjectPayment, ResponseProjectRemarks, ResponseProjectRemarksImg, ResponseProjectsByFund, ResponseProjectsById, ResponseProjectsOverview, ResponseProjectsSectorWithAppropriation, ResponseProjectsStatsCategory, ResponseProjectsStatsTypes, ResponseProjectsTop10Appropriation, ResponseProjectsTop10Awared, ResponseViewProjectsById}, settings::{ResponseBarangays, ResponseSector, ResponseSustainableDevelopmentGoals}}};
use tokio::try_join;

// GET
pub async fn get_projects_infra_code_by_projects_id(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {

    let mut redis = Redis::new(format!("get_projects_infra_code_by_projects_id_{}", id), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = projects_infra_code::Entity::find()
        .filter(projects_infra_code::Column::ProjectId.eq(id))
        .one(&db.db_connection).await.unwrap();

    if response.is_some() {
        redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();
    }

    return (StatusCode::OK, Json(response))
}

#[derive(FromQueryResult, Clone)]
struct GetProjectsById {
    pub projects_id: i32,
    pub projects_name: String,
    pub projects_code: String,
    pub project_year: i32,
    pub projects_status: String,
    pub projects_appropriation: Option<i32>,
    pub projects_approved_budget_contract: Option<i32>,
    pub projects_accomplished: Option<i16>,
    pub projects_remarks: Option<String>,
    pub projects_contract_cost: Option<i32>,
    pub projects_start_date: Option<String>,
    pub projects_target_date: Option<String>,
    pub projects_barangay: Option<Vec<i32>>,
    pub projects_sdg: Option<Vec<i32>>,
    pub project_sector: Option<Vec<i32>>,
    pub s_types_name: Option<String>,
    pub s_category_name: Option<String>,
    pub s_sof_name: Option<String>,
    pub s_incharge_name: Option<String>,
    pub s_takers_name: Option<String>,
    pub contractor_name: Option<String>,
    pub infra_code: Option<String>
}

pub async fn get_projects_by_id(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {

    let mut redis = Redis::new(format!("get_projects_by_id_{}", id), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let get_projects_by_id: Option<GetProjectsById> = projects::Entity::find_by_id(id)
        .filter(projects::Column::IsDisposed.eq(false))
        .left_join(settings_type::Entity)
        .left_join(settings_categories::Entity)
        .left_join(settings_sof::Entity)
        .left_join(settings_incharge::Entity)
        .left_join(settings_takers::Entity)
        .left_join(contractors::Entity)
        .join_rev(sea_orm::JoinType::LeftJoin, projects_infra_code::Relation::Projects.def())
        .select_only()
        .column_as(projects::Column::Id, "projects_id")
        .column_as(projects::Column::ProjectName, "projects_name")
        .column_as(projects::Column::ProjectCode, "projects_code")
        .column_as(projects::Column::ProjectYear, "project_year")
        .column_as(projects::Column::ProjectStatus, "projects_status")
        .column_as(projects::Column::Appropriation, "projects_appropriation")
        .column_as(projects::Column::ApprovedBudgetContact, "projects_approved_budget_contract")
        .column_as(projects::Column::Accomplished, "projects_accomplished")
        .column_as(projects::Column::Remarks, "projects_remarks")
        .column_as(projects::Column::ContractCost, "projects_contract_cost")
        .column_as(projects::Column::StartDate, "projects_start_date")
        .column_as(projects::Column::TargetDate, "projects_target_date")
        .column_as(projects::Column::Barangays, "projects_barangay")
        .column_as(projects::Column::SustainableDevelopmentGoals, "projects_sdg")
        .column_as(projects::Column::Sector, "project_sector")
        .column_as(settings_type::Column::Name, "s_types_name")
        .column_as(settings_categories::Column::Name, "s_category_name")
        .column_as(settings_sof::Column::Name, "s_sof_name")
        .column_as(settings_incharge::Column::Name, "s_incharge_name")
        .column_as(settings_takers::Column::Name, "s_takers_name")
        .column_as(contractors::Column::Name, "contractor_name")
        .column_as(projects_infra_code::Column::ProjectCode, "infra_code")
        .into_model()
        .one(&db.db_connection).await.unwrap();


    let get_projects_payment = projects_payment::Entity::find()
        .filter(projects_payment::Column::ProjectId.eq(id))
        .order_by_desc(projects_payment::Column::Id)
        .into_model::<ResponseProjectPayment>().all(&db.db_connection)
        .await.unwrap();   

    let parse_projects_monitoring_remarks: Vec<ResponseProjectRemarks> = {

        let projects_monitoring_remarks = projects_monitoring_remarks::Entity::find()
            .filter(projects_monitoring_remarks::Column::ProjectId.eq(id))
            .order_by_desc(projects_monitoring_remarks::Column::Id)
            .all(&db.db_connection).await.unwrap();


        projects_monitoring_remarks.iter().map(|v| ResponseProjectRemarks {
            id: v.id,
            remarks: v.remarks.to_owned(),
            remarks_date: v.remarks_date.to_owned(),
            project_id: v.project_id,
        }).collect()
    };
    
    
    let parse_projects_monitoring_img: Vec<ResponseProjectRemarksImg> = {

        let get_remarks_id = parse_projects_monitoring_remarks.iter().map(|v| v.id);

        let projects_monitoring_img = projects_monitoring_img::Entity::find()
            .filter(projects_monitoring_img::Column::RemarksId.is_in(get_remarks_id))
            .all(&db.db_connection).await.unwrap();

        projects_monitoring_img.iter().map(|v| ResponseProjectRemarksImg {
            id: v.id,
            images_key: v.images_key.to_owned(),
            images_original_name: v.images_original_name.to_owned(),
            remarks_id: v.remarks_id
        }).collect()
    };
    
    if get_projects_by_id.is_none() {
        return (StatusCode::OK, Json::default());
    }

    let unwrap_projects_by_id = get_projects_by_id.unwrap();

    let get_some_settings = try_join!(
        settings_barangay::Entity::find()
            .filter(settings_barangay::Column::Id.is_in(unwrap_projects_by_id.clone().projects_barangay.unwrap_or_default()))
            .all(&db.db_connection),
        settings_sdg::Entity::find()
            .filter(settings_sdg::Column::Id.is_in(unwrap_projects_by_id.clone().projects_sdg.unwrap_or_default()))
            .all(&db.db_connection),
        settings_sector::Entity::find()
            .filter(settings_sector::Column::Id.is_in(unwrap_projects_by_id.clone().project_sector.unwrap_or_default()))
            .all(&db.db_connection)
    ).unwrap();

    let (get_barangay, get_sdg, get_sector) = get_some_settings;

    let parsed_barangay: Vec<ResponseBarangays> = get_barangay.into_iter().map(|val| ResponseBarangays {
        id: val.id,
        barangay_type: match val.barangay_type {
            Some(b_type) => match b_type {
                settings_barangay::BarangayType::Rural => Some(services::request::settings::RequestBarangayType::Rural),
                settings_barangay::BarangayType::Urban => Some(services::request::settings::RequestBarangayType::Urban),
            },
            None => None
        },
        is_poblacion: val.is_poblacion,
        name: val.name
    }).collect();

    let parsed_get_sdg: Vec<ResponseSustainableDevelopmentGoals> = get_sdg.into_iter().map(|val| ResponseSustainableDevelopmentGoals { id: val.id, name: val.name }).collect();
    let parsed_get_sector: Vec<ResponseSector> = get_sector.into_iter().map(|val| ResponseSector { id: val.id, name: val.name }).collect();

    let response_projects_by_id = ResponseProjectsById { 
        projects_barangay: Some(parsed_barangay),
        projects_sdg: Some(parsed_get_sdg),
        projects_sector: Some(parsed_get_sector),
        projects_id: unwrap_projects_by_id.projects_id,
        projects_name: unwrap_projects_by_id.projects_name,
        projects_code: unwrap_projects_by_id.projects_code,
        project_year: unwrap_projects_by_id.project_year,
        projects_status: unwrap_projects_by_id.projects_status,
        projects_appropriation: unwrap_projects_by_id.projects_appropriation,
        projects_approved_budget_contract: unwrap_projects_by_id.projects_approved_budget_contract,
        projects_accomplished: unwrap_projects_by_id.projects_accomplished,
        projects_remarks: unwrap_projects_by_id.projects_remarks,
        projects_contract_cost: unwrap_projects_by_id.projects_contract_cost,
        projects_start_date: unwrap_projects_by_id.projects_start_date,
        projects_target_date: unwrap_projects_by_id.projects_target_date,
        s_types_name: unwrap_projects_by_id.s_types_name,
        s_category_name: unwrap_projects_by_id.s_category_name,
        s_sof_name: unwrap_projects_by_id.s_sof_name,
        s_incharge_name: unwrap_projects_by_id.s_incharge_name,
        s_takers_name: unwrap_projects_by_id.s_takers_name,
        contractor_name: unwrap_projects_by_id.contractor_name,
        infra_code: unwrap_projects_by_id.infra_code,
    };

    let response = ResponseViewProjectsById {
        projects_info: response_projects_by_id,
        projects_remarks: parse_projects_monitoring_remarks,
        projects_remarks_image: parse_projects_monitoring_img,
        projects_payment: get_projects_payment
    };

    redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(response))

}

pub async fn get_projects_stats_overview(
   Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_projects_stats_overview".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let get_all_projects_count = projects::Entity::find()
        .filter(projects::Column::IsDisposed.is_null())
        .select_only()
        .expr_as(Expr::count(Expr::col(projects::Column::Id)), "total_projects")
        .expr_as(Func::sum(
            Expr::case(
                Expr::col(projects::Column::ProjectStatus).cast_as("text").eq(ProjectStatus::NotYetStarted), 1).finally(0)
            ), "total_unimplemented")
        .expr_as(Func::sum(
            Expr::case(
                Expr::col(projects::Column::ProjectStatus).cast_as("text").eq(ProjectStatus::Preparation), 1).finally(0)
        ), "total_preparing")
        .expr_as(Func::sum(
            Expr::case(
                Expr::col(projects::Column::ProjectStatus).cast_as("text").eq(ProjectStatus::Bidded), 1).finally(0)
        ), "total_bidded")
        .expr_as(Func::sum(
            Expr::case(
                Expr::col(projects::Column::ProjectStatus).cast_as("text").eq(ProjectStatus::Bidding), 1).finally(0)
        ), "total_bidding")
        .expr_as(Func::sum(
            Expr::case(
                Expr::col(projects::Column::ProjectStatus).cast_as("text").eq(ProjectStatus::OnGoing), 1).finally(0)
        ), "total_ongoing")
        .expr_as(Func::sum(
            Expr::case(
                Expr::col(projects::Column::ProjectStatus).cast_as("text").eq(ProjectStatus::Completed), 1).finally(0)
        ), "total_completed")
        .into_model::<ResponseProjectsOverview>().one(&db.db_connection).await.unwrap();
    
    if get_all_projects_count.as_ref().is_some_and(|v: &ResponseProjectsOverview| v.total_projects.is_some_and(|tp| tp > 0)) {
        redis.stored_value(&serde_json::to_string(&get_all_projects_count).unwrap(), None).await.unwrap();
    }

    (StatusCode::OK, Json(get_all_projects_count))

}

pub async fn get_projects_stats_by_types(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {
    
    let mut redis = Redis::new("get_projects_stats_by_types".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = settings_type::Entity::find()
        .join_rev(sea_orm::JoinType::LeftJoin, projects::Relation::SettingsType.def())
        .filter(projects::Column::IsDisposed.eq(false))
        .select_only()
        .select_column_as(settings_type::Column::Name, "name")
        .expr_as(Func::count(Expr::col((projects::Entity, projects::Column::Id))), "projects_total")
        .group_by(settings_type::Column::Name)
        .order_by_desc(Expr::col("projects_total"))
        .into_model::<ResponseProjectsStatsTypes>()
        .all(&db.db_connection).await.unwrap();

    if response.len().gt(&0) {
        redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();
    }
    (StatusCode::OK, Json(response))
}

pub async fn get_projects_stats_by_category(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_projects_stats_by_category".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data))
    }

    let response = settings_categories::Entity::find()
        .join_rev(sea_orm::JoinType::LeftJoin, projects::Relation::SettingsCategories.def())
        .select_only()
        .select_column_as(settings_categories::Column::Name, "name")
        .expr_as(Func::count(Expr::col((projects::Entity, projects::Column::Id))), "projects_total")
        .group_by(settings_categories::Column::Name)
        .order_by_desc(Expr::col("projects_total"))
        .into_model::<ResponseProjectsStatsCategory>()
        .all(&db.db_connection).await.unwrap();

    if !response.is_empty() {
        redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();
    }

    (StatusCode::OK, Json(response))
}


#[derive(FromQueryResult)]
struct ProjectsAppropriationBySector {
    pub appropriation: Option<i32>,
    pub sector: Option<Vec<i32>>,
}

pub async fn get_projects_appropriation_by_sector(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_projects_appropriation_by_sector".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = projects::Entity::find()
        .filter(projects::Column::Sector.is_not_null())
        .select_only()
        .select_column_as(projects::Column::Appropriation, "appropriation")
        .select_column(projects::Column::Sector)
        .into_model::<ProjectsAppropriationBySector>()
        .all(&db.db_connection).await.unwrap();

    let mut projects_appropriation: HashMap<String, ResponseProjectsSectorWithAppropriation> = HashMap::new();

    let mut all_sector_ids: HashSet<i32> = HashSet::new();
    for value in &response {
        if let Some(sectors) = &value.sector {
            all_sector_ids.extend(sectors);
        }
    }

    let sectors_map: HashMap<i32, String> = if !all_sector_ids.is_empty() {
        settings_sector::Entity::find()
            .filter(settings_sector::Column::Id.is_in(all_sector_ids))
            .all(&db.db_connection).await.unwrap()
            .into_iter()
            .map(|sector| (sector.id, sector.name))
            .collect()
    } else {
        HashMap::new()
    };

    for value in &response {
        if let Some(sectors) = &value.sector {
            for sector_id in sectors {
                if let Some(sector_name) = sectors_map.get(sector_id) {
                    let appropriation = value.appropriation.unwrap_or(0);
                    
                    projects_appropriation.entry(sector_name.clone())
                        .and_modify(|existing| {
                            existing.value = existing.value + appropriation;
                        })
                        .or_insert(ResponseProjectsSectorWithAppropriation {
                            name: sector_name.clone(),
                            value: appropriation,
                        });
                }
            }
        }
    }

    let response =  projects_appropriation.into_values().collect::<Vec<ResponseProjectsSectorWithAppropriation>>();

    redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();

    return (StatusCode::OK, Json(response))

}


pub async fn get_projects_top_10_by_appropriation(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_projects_top_10_by_appropriation".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = projects::Entity::find()
        .filter(projects::Column::Appropriation.is_not_null())
        .select_only()
        .column_as(projects::Column::Id, "id")
        .column_as(projects::Column::ProjectName, "name")
        .column_as(projects::Column::ProjectCode, "project_code")
        .column_as(projects::Column::Appropriation, "appropriation")
        .order_by_desc(projects::Column::Appropriation)
        .limit(10)
        .into_model::<ResponseProjectsTop10Appropriation>()
        .all(&db.db_connection).await.unwrap();

    redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(response))

}

pub async fn get_projects_top_10_by_awarded_contractors(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_projects_top_10_by_awarded_contractors".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = contractors::Entity::find()
        .join_rev(sea_orm::JoinType::LeftJoin, projects::Relation::Contractors.def())
        .select_only()
        .select_column_as(contractors::Column::Id, "constructor_id")
        .select_column_as(contractors::Column::Name, "name")
        .expr_as(Expr::col((projects::Entity, projects::Column::Id)).count(), "total_projects")
        .expr_as(Expr::col((projects::Entity, projects::Column::Appropriation)).sum(), "total_appropriation")
        .group_by(contractors::Column::Name)
        .group_by(contractors::Column::Id)
        .order_by_desc(Expr::col("total_appropriation"))
        .having(Expr::col((projects::Entity, projects::Column::Id)).count().gt(0))
        .limit(10)
        .into_model::<ResponseProjectsTop10Awared>()
        .all(&db.db_connection).await.unwrap();

    if response.len().gt(&0) {
        redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();
    }


    (StatusCode::OK, Json(response))
}

pub async fn get_projects_by_fund(
    Extension(db): Extension<Arc<DB>>,
    Path(id): Path<i32>
) -> impl IntoResponse {

    let mut redis = Redis::new(format!("get_projects_by_fund_{}", id), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let projects_condition = if id == 0 {
        projects::Column::ProjectSofId.ne(id)
    } else {
        projects::Column::ProjectSofId.eq(id)
    };
    
    let response: Vec<ResponseProjectsByFund> = projects::Entity::find()
        .filter(Condition::all()
            .add(projects_condition)
            .add(projects::Column::IsDisposed.eq(false))
        )
        .left_join(contractors::Entity)
        .select_column_as(projects::Column::Id, "projects_id")
        .select_column_as(projects::Column::ProjectName, "projects_name")
        .select_column_as(projects::Column::ProjectCode, "project_code")
        .select_column_as(projects::Column::ProjectStatus, "project_status")
        .expr_as(Func::coalesce([Expr::col(projects::Column::ContractCost).into(), Expr::val(0).into()]), "contract_cost")
        .expr_as(Expr::col((contractors::Entity, contractors::Column::Id)), "contractor_id")
        .expr_as(Expr::col((contractors::Entity, contractors::Column::Name)), "contractor_name")
        .into_model()
        .all(&db.db_connection).await.unwrap();

    if response.len().gt(&0) {
        redis.stored_value(&serde_json::to_string(&response).unwrap(), None).await.unwrap();
    }

    (StatusCode::OK, Json(response))
}

#[derive(Serialize, Deserialize)]
struct PrepareProjectsForAdd {
    s_barangay: Vec<settings_barangay::Model>,
    contractors: Vec<contractors::Model>,
    s_type: Vec<settings_type::Model>,
    s_categories: Vec<settings_categories::Model>,
    s_sof: Vec<settings_sof::Model>,
    s_incharge: Vec<settings_incharge::Model>,
    s_sdg: Vec<settings_sdg::Model>,
    s_sector: Vec<settings_sector::Model>,
    s_takers: Vec<settings_takers::Model>
}

pub async fn get_prepare_add_projects(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("get_prepare_add_projects".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }

    let response = try_join!(
        settings_barangay::Entity::find().order_by_asc(settings_barangay::Column::Name).all(&db.db_connection),
        contractors::Entity::find().order_by_asc(contractors::Column::Name).all(&db.db_connection),
        settings_type::Entity::find().order_by_asc(settings_type::Column::Name).all(&db.db_connection),
        settings_categories::Entity::find().order_by_asc(settings_categories::Column::Name).all(&db.db_connection),
        settings_sof::Entity::find().order_by_asc(settings_sof::Column::Name).all(&db.db_connection),
        settings_incharge::Entity::find().order_by_asc(settings_incharge::Column::Name).all(&db.db_connection),
        settings_sdg::Entity::find().order_by_asc(settings_sdg::Column::Name).all(&db.db_connection),
        settings_sector::Entity::find().order_by_asc(settings_sector::Column::Name).all(&db.db_connection),
        settings_takers::Entity::find().order_by_asc(settings_takers::Column::Name).all(&db.db_connection)
    ).unwrap();

    let prepare_response = PrepareProjectsForAdd {
        s_barangay: response.0,
        contractors: response.1,
        s_type: response.2,
        s_categories: response.3,
        s_sof: response.4,
        s_incharge: response.5,
        s_sdg: response.6,
        s_sector: response.7,
        s_takers: response.8
    };

    redis.stored_value(&serde_json::to_string(&prepare_response).unwrap(), None).await.unwrap();

    (StatusCode::OK, Json(prepare_response))
}

// POST
pub async fn add_projects(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestAddProjects>
) -> impl IntoResponse {
    
    let find_projects = projects::Entity::find().filter(projects::Column::ProjectCode.eq(&request.project_code)).one(&db.db_connection).await.unwrap();

    if find_projects.is_some() {
        return (StatusCode::CONFLICT, format!(""))
    }

    let make_add = projects::ActiveModel {
        project_year: Set(request.project_year),
        project_name: Set(request.project_name),
        project_code: Set(request.project_code),
        project_status: Set({
            match request.project_status {
                Some(req) => match req {
                    services::request::projects::ProjectStatus::NotYetStarted => Some(ProjectStatus::NotYetStarted),
                    services::request::projects::ProjectStatus::Preparation => Some(ProjectStatus::Preparation),
                    services::request::projects::ProjectStatus::Bidding => Some(ProjectStatus::Bidding),
                    services::request::projects::ProjectStatus::Bidded => Some(ProjectStatus::Bidded),
                    services::request::projects::ProjectStatus::OnGoing => Some(ProjectStatus::OnGoing),
                    services::request::projects::ProjectStatus::Completed => Some(ProjectStatus::Completed),
                    services::request::projects::ProjectStatus::Suspended => Some(ProjectStatus::Suspended),
                },
                None => None,
            }
        }),
        barangays: Set(request.barangays),
        appropriation: Set(request.appropriation),
        contractor_id: Set(request.contractor_id),
        contract_cost: Set(request.contract_cost),
        start_date: Set(request.start_date),
        calendar_days: Set(request.calendar_days),
        time_extensions: Set(request.time_extensions),
        target_date: Set(request.target_date),
        project_type_id: Set(request.project_type_id),
        project_category_id: Set(request.project_category_id),
        project_sof_id: Set(request.project_sof_id),
        project_incharge_id: Set(request.project_incharge_id),
        sustainable_development_goals: Set(request.sustainable_development_goals),
        sector: Set(request.sector),
        project_takers_id: Set(request.project_takers_id),
        accomplished: Set(request.accomplished),
        remarks: Set(request.remarks),
        prepared_users_id: Set(request.prepared_users_id),
        ..Default::default()
    };

    let response = projects::Entity::insert(make_add).exec(&db.db_connection).await.unwrap();

    Redis::new("get_projects_stats_overview".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![
            "get_projects_stats_by_types",
            "get_projects_stats_by_category",
            "get_projects_appropriation_by_sector",
            "get_projects_top_10_by_appropriation",
            "get_projects_top_10_by_awarded_contractors",
            "get_prepare_add_projects",
            "get_projects_by_fund_18" // DEFAULT FOR GET PROJECTS BY BARANGAY
        ]).await.unwrap();

    (StatusCode::CREATED, format!("{}", response.last_insert_id))

}

pub async fn update_projects(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpdateProjects>
) -> impl IntoResponse {

    let get_projects_by_id = projects::Entity::find()
        .filter(projects::Column::Id.eq(request.id)).one(&db.db_connection).await.unwrap();

    if get_projects_by_id.is_none() {
        return (StatusCode::NOT_FOUND, format!(""))
    }

    let mut make_it_active = get_projects_by_id.unwrap().into_active_model();

    make_it_active.project_year = Set(request.project_year);
    make_it_active.project_name = Set(request.project_name);
    make_it_active.project_code = Set(request.project_code);
    make_it_active.project_status = Set({
        match request.project_status {
            Some(req) => match req {
                services::request::projects::ProjectStatus::NotYetStarted => Some(ProjectStatus::NotYetStarted),
                services::request::projects::ProjectStatus::Preparation => Some(ProjectStatus::Preparation),
                services::request::projects::ProjectStatus::Bidding => Some(ProjectStatus::Bidding),
                services::request::projects::ProjectStatus::Bidded => Some(ProjectStatus::Bidded),
                services::request::projects::ProjectStatus::OnGoing => Some(ProjectStatus::OnGoing),
                services::request::projects::ProjectStatus::Completed => Some(ProjectStatus::Completed),
                services::request::projects::ProjectStatus::Suspended => Some(ProjectStatus::Suspended),
            },
            None => None,
        }
    });
    make_it_active.barangays = Set(request.barangays);
    make_it_active.appropriation = Set(request.appropriation);
    make_it_active.contractor_id = Set(request.contractor_id);
    make_it_active.contract_cost = Set(request.contract_cost);
    make_it_active.start_date = Set(request.start_date);
    make_it_active.calendar_days = Set(request.calendar_days);
    make_it_active.time_extensions = Set(request.time_extensions);
    make_it_active.target_date = Set(request.target_date);
    make_it_active.project_type_id = Set(request.project_type_id);
    make_it_active.project_category_id = Set(request.project_category_id);
    make_it_active.project_sof_id = Set(request.project_sof_id);
    make_it_active.project_incharge_id = Set(request.project_incharge_id);
    make_it_active.sustainable_development_goals = Set(request.sustainable_development_goals);
    make_it_active.sector = Set(request.sector);
    make_it_active.project_takers_id = Set(request.project_takers_id);
    make_it_active.accomplished = Set(request.accomplished);
    make_it_active.remarks = Set(request.remarks);
    make_it_active.prepared_users_id = Set(request.prepared_users_id);

    let response = make_it_active.save(&db.db_connection).await.unwrap();

    Redis::new(format!("get_projects_by_id_{}", request.id), db.redis_connection.clone())
        .remove_multi_value(vec![
            "get_projects_stats_overview",
            "get_projects_stats_by_types",
            "get_projects_stats_by_category",
            "get_projects_appropriation_by_sector",
            "get_projects_top_10_by_appropriation",
            "get_projects_top_10_by_awarded_contractors",
            "get_prepare_add_projects",
            "get_projects_by_fund_18" // DEFAULT FOR GET PROJECTS BY BARANGAY
        ]).await.unwrap();

    (StatusCode::CREATED, format!("{}", response.id.into_value().unwrap()))

}

pub async fn dispose_projects_by_id(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDisposedProjectsById>
) -> impl IntoResponse {

    if user::Entity::find().filter(user::Column::Password.eq(request.hash_password())).one(&db.db_connection).await.unwrap().is_none() {
        return (StatusCode::UNPROCESSABLE_ENTITY, format!(""));
    }

    let get_projects_by_id = projects::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if get_projects_by_id.is_none() {
        return (StatusCode::NOT_FOUND, format!(""));
    }

    let mut projects_model = get_projects_by_id.unwrap().into_active_model();

    projects_model.is_disposed = Set(Some(true));

    let response = projects_model.save(&db.db_connection).await.unwrap();

    Redis::new("get_projects_stats_overview".to_string(), db.redis_connection.clone())
        .remove_multi_value(vec![
            "get_projects_stats_by_types",
            "get_projects_stats_by_category",
            "get_projects_appropriation_by_sector",
            "get_projects_top_10_by_appropriation",
            "get_projects_top_10_by_awarded_contractors",
            "get_prepare_add_projects",
            "get_projects_by_fund_18", // DEFAULT FOR GET PROJECTS BY BARANGAY,
            &format!("get_projects_by_id_{}", request.id),
        ]).await.unwrap();

    (StatusCode::CREATED, format!("{}", response.id.into_value().unwrap()))

}

pub async fn upsert_projects_infra_code(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestUpsertProjectsInfraCode>
) -> impl IntoResponse {

    let find_projects = projects::Entity::find_by_id(request.projects_id).one(&db.db_connection).await.unwrap();

    if find_projects.is_none() {
        return (StatusCode::NOT_FOUND, format!(""));
    }

    let make_upsert = projects_infra_code::ActiveModel {
        id: match request.id {
            Some(id) => Set(id),
            None => NotSet
        },
        project_code: Set(request.projects_code),
        project_id: Set(request.projects_id)
    };

    Redis::new(format!("get_projects_by_id_{}", request.projects_id), db.redis_connection.clone())
        .remove_multi_value(vec![
            &format!("get_projects_infra_code_by_projects_id_{}", request.projects_id)
        ]).await.unwrap();

    let response = make_upsert.save(&db.db_connection).await.unwrap();

    (StatusCode::CREATED, format!("{}", response.id.into_value().unwrap()))

}

pub async fn delete_projects_remarks(
    Extension(db): Extension<Arc<DB>>,
    Json(request): Json<RequestDeleteProjectRemarks>
) -> impl IntoResponse {

    if user::Entity::find().filter(user::Column::Password.eq(request.hash_password())).one(&db.db_connection).await.unwrap().is_none() {
        return (StatusCode::NOT_ACCEPTABLE, format!(""));
    }

    let find_get_projects = projects_monitoring_remarks::Entity::find_by_id(request.id).one(&db.db_connection).await.unwrap();

    if find_get_projects.is_none() {
        return (StatusCode::NOT_FOUND, format!(""));
    }

    Redis::new(format!("get_projects_by_id_{}", find_get_projects.as_ref().unwrap()
        .project_id), db.redis_connection.clone())
        .remove_value_if_exists().await.unwrap();

    projects_monitoring_img::Entity::delete_many()
        .filter(projects_monitoring_img::Column::RemarksId.eq(request.id)).exec(&db.db_connection).await.unwrap();

    find_get_projects.unwrap().delete(&db.db_connection).await.unwrap();



    (StatusCode::CREATED, format!(""))
}

