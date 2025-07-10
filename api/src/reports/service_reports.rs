use std::{collections::HashSet, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use entity::{contractors, projects_payment, settings::{settings_sof, settings_takers, settings_type}};
use ::entity::projects;
use services::{db_connection::DB, redis::Redis, response::reports::{ResponseSummaryFinancialStatusProjects, ResponseSummaryFinancialStatusProjectsOverview, ResponseSummaryFinancialStatusReports, ResponseSummaryFinancialStatusReportsOverview, ResponseSummaryImplementationByYear, ResponseSummaryImplementationByYearFullOverview, ResponseSummaryImplementationByYearOverview, ResponseSummaryListOfProjects, ResponseSummaryListOfProjectsOverview, ResponseSummaryProjectSavingsReport, ResponseSummaryProjectSavingsReportOverview, ResponseSummaryProjectsPerType, ResponseSummaryProjectsPerTypeOverview, ResponseSummarySlippageReport, ResponseSummarySlippageReportOverview}};
use sea_orm::{prelude::*, sea_query::{ExprTrait, Func}, Condition, QueryOrder, QuerySelect, SelectColumns};
use tokio::try_join;

pub async fn summary_list_of_projects(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("summary_list_of_projects".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data));
    }
 
    let get_projects = projects::Entity::find()
        .filter(projects::Column::IsDisposed.eq(false))
        .left_join(contractors::Entity)
        .left_join(settings_takers::Entity)
        .left_join(settings_sof::Entity)
        .select_only()
        .select_column_as(projects::Column::Id, "project_id")
        .select_column_as(projects::Column::ProjectName, "project_name")
        .select_column_as(projects::Column::ProjectCode, "project_code")
        .select_column_as(projects::Column::Appropriation, "project_appropriation")
        .select_column_as(projects::Column::StartDate, "project_bid_date")
        .select_column_as(projects::Column::ContractCost, "project_contract_cost")
        .select_column_as(contractors::Column::Id, "contractor_id")
        .select_column_as(contractors::Column::Name, "contractor_name")
        .select_column_as(settings_sof::Column::Id, "sof_id")
        .select_column_as(settings_sof::Column::Name, "sof_name")
        .select_column_as(projects::Column::ProjectStatus, "project_status")
        .select_column_as(projects::Column::Accomplished, "project_accomplished")
        .select_column_as(settings_takers::Column::Id, "project_taker_id")
        .select_column_as(settings_takers::Column::Name, "project_taker_name")
        .select_column_as(projects::Column::ProjectYear, "project_year")
        .select_column_as(projects::Column::Remarks, "project_remarks")
        .into_model::<ResponseSummaryListOfProjects>()
        .all(&db.db_connection).await.unwrap();


    let total_records: usize = get_projects.len();
    let total_appropriation: i64 = get_projects.iter().map(|v| v.project_appropriation.unwrap_or(0) as i64).sum();
    let total_contract_cost: i64 = get_projects.iter().map(|v| v.project_contract_cost.unwrap_or(0) as i64).sum();

    let make_response = ResponseSummaryListOfProjectsOverview {
        summary_list_of_reports: get_projects,
        total_records: total_records,
        total_appropriation: total_appropriation,
        total_contract_cost: total_contract_cost
    };

    redis.stored_value(&serde_json::to_string(&make_response).unwrap()).await.unwrap();
    

    return (StatusCode::OK, Json(make_response))
}

pub async fn summary_implementation_by_year (
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {
    
    let mut redis = Redis::new("summary_implementation_by_year".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data))
    }

    let response = projects::Entity::find()
        .filter(Condition::all()
            .add(projects::Column::ProjectStatus.is_not_null())
            .add(projects::Column::IsDisposed.eq(false))    
        )
        .select_only()
        .select_column_as(projects::Column::ProjectYear, "project_year")
        .select_column_as(projects::Column::ProjectStatus, "project_status")
        .expr_as(Expr::col(projects::Column::Id).count(), "total_projects")
        .expr_as(Expr::col(projects::Column::Appropriation).sum(), "total_appropriation")
        .group_by(projects::Column::ProjectYear)
        .group_by(projects::Column::ProjectStatus)
        .order_by_asc(projects::Column::ProjectYear)
        .into_model::<ResponseSummaryImplementationByYear>()
        .all(&db.db_connection).await.unwrap();

    let get_year: Vec<i32> = response.iter().map(|v| v.project_year.to_owned()).collect();

    let parse_year: HashSet<i32> = HashSet::from_iter(get_year);

    let parse_projects: Vec<ResponseSummaryImplementationByYearOverview> = parse_year.iter().map(|v| {

        let get_data_by_year: Vec<ResponseSummaryImplementationByYear> = response.iter().filter_map(|project| {
            let clone_project: ResponseSummaryImplementationByYear = project.to_owned();
            
            if clone_project.project_year == v.to_owned() {
                Some(clone_project)
            } else {
                None
            }
        }).collect();

       let make_data = ResponseSummaryImplementationByYearOverview {
            year: *v,
            data: get_data_by_year
       };
        
        make_data
    }).collect();

    let make_response = ResponseSummaryImplementationByYearFullOverview {
        projects_data: parse_projects.clone(),
        total_status: 7,
        total_projects: parse_projects.iter().fold(0, |acc: i64, n: &ResponseSummaryImplementationByYearOverview| {
            acc + n.data.iter().fold(0, |acc, n: &ResponseSummaryImplementationByYear| acc + n.total_projects) as i64
        }),
        total_appropriation: parse_projects.iter().fold(0, |acc: i64, n: &ResponseSummaryImplementationByYearOverview| {
            acc + n.data.iter().fold(0, |acc, n: &ResponseSummaryImplementationByYear| acc + n.total_appropriation) as i64
        }),
    };

    redis.stored_value(&serde_json::to_string(&make_response).unwrap()).await.unwrap();

    (StatusCode::OK, Json(make_response))

}

pub async fn summary_projects_per_type (
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {
    
    let mut redis = Redis::new("summary_projects_per_type".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data))
    }

    let get_projects_type = settings_type::Entity::find()
        .join_rev(sea_orm::JoinType::LeftJoin, projects::Relation::SettingsType.def())
        .select_only()
        .column_as(settings_type::Column::Name, "project_type")
        .expr_as(Expr::col((projects::Entity, projects::Column::Id)).count(), "total_projects")
        .expr_as(Func::sum(Expr::case(
            Expr::col((projects::Entity, projects::Column::Appropriation)).is_not_null(),
        Expr::col((projects::Entity, projects::Column::Appropriation))).finally(Expr::value(0))), "total_appropriation")
        .group_by(settings_type::Column::Name)
        .into_model::<ResponseSummaryProjectsPerType>()
        .all(&db.db_connection).await.unwrap();


    let make_response = ResponseSummaryProjectsPerTypeOverview {
        data: get_projects_type.clone(),
        total_types: get_projects_type.iter().count(),
        total_projects: get_projects_type.iter().fold(0, |acc, n| acc + n.total_projects),
        total_appropriation: get_projects_type.iter().fold(0, |acc, n| acc + n.total_appropriation),
    };

    redis.stored_value(&serde_json::to_string(&make_response).unwrap()).await.unwrap();

    (StatusCode::OK, Json(make_response))
}

pub async fn summary_project_savings_report (
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("summary_project_savings_report".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data))
    }
    
    let get_response = projects::Entity::find()
        .filter(projects::Column::IsDisposed.eq(false))
        .left_join(contractors::Entity)
        .left_join(settings_takers::Entity)
        .select_only()
        .select_column_as(projects::Column::ProjectName, "project_name")
        .select_column_as(projects::Column::ProjectCode, "project_code")
        .select_column_as(projects::Column::ProjectStatus, "project_status")
        .select_column_as(contractors::Column::Name, "contractor_name")
        .select_column_as(settings_takers::Column::Name, "taker_name")
        .select_column_as(projects::Column::ApprovedBudgetContact, "approved_budget_contract")
        .select_column_as(projects::Column::Appropriation, "appropriation")
        .select_column_as(projects::Column::ContractCost, "contract_cost")
        .expr_as(Expr::case(
            Expr::col(projects::Column::Appropriation).is_not_null(), Expr::col(projects::Column::Appropriation)).finally(Expr::value(0)
        ).sub(Expr::case(
            Expr::col(projects::Column::ContractCost).is_not_null(), Expr::col(projects::Column::ContractCost)).finally(Expr::value(0)
        )), "savings")
        .into_model::<ResponseSummaryProjectSavingsReport>()
        .all(&db.db_connection).await.unwrap();

    let make_response = ResponseSummaryProjectSavingsReportOverview {
        data: get_response.clone(),
        total_records: get_response.len(),
        total_appropriation: get_response.iter().fold(0, |acc, v| acc + v.appropriation.unwrap_or(0) as i64),
        total_abc: get_response.iter().fold(0, |acc, v| acc + v.approved_budget_contract.unwrap_or(0) as i64),
        total_contract_cost: get_response.iter().fold(0, |acc, v| acc + v.contract_cost.unwrap_or(0) as i64),
        total_savings: get_response.iter().fold(0, |acc, v| acc + v.savings as i64),
    };

    redis.stored_value(&serde_json::to_string(&make_response).unwrap()).await.unwrap();

    (StatusCode::OK, Json(make_response))
}

pub async fn summary_slippage_report(
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("summary_slippage_report".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data))
    }

    let get_projects = projects::Entity::find()
        .filter(Condition::all()
            .add(projects::Column::IsDisposed.eq(false))
            .add(projects::Column::ProjectYear.is_not_null())
            .add(projects::Column::ProjectSofId.is_not_null())
            .add(projects::Column::Sector.is_not_null())
            .add(projects::Column::ProjectTypeId.is_not_null())
            .add(projects::Column::ProjectCategoryId.is_not_null())
            .add(projects::Column::Barangays.is_not_null())
            .add(projects::Column::SustainableDevelopmentGoals.is_not_null())
            .add(projects::Column::TargetDate.is_not_null())
            .add(Condition::all()
                    .add(projects::Column::ProjectStatus.ne("Completed"))
                    .add(projects::Column::ProjectStatus.ne("Suspended"))
                )
        )
        .left_join(contractors::Entity)
        .left_join(settings_takers::Entity)
        .select_only()
        .select_column_as(projects::Column::ProjectName, "project_name")
        .select_column_as(contractors::Column::Name, "contractor_name")
        .select_column_as(settings_takers::Column::Name, "taker_name")
        .select_column_as(projects::Column::StartDate, "start_date")
        .select_column_as(projects::Column::TargetDate, "target_date")
        .expr_as(Expr::current_date().sub(Expr::col(projects::Column::TargetDate).cast_as("date")), "days_lapse")
        .select_column_as(projects::Column::Remarks, "remarks")
        .into_model::<ResponseSummarySlippageReport>()
        .all(&db.db_connection).await.unwrap();

    let make_response = ResponseSummarySlippageReportOverview {
        data: get_projects.clone(),
        total_records: get_projects.len()
    };

    redis.stored_value(&serde_json::to_string(&make_response).unwrap()).await.unwrap();

    (StatusCode::OK, Json(make_response))

}

pub async fn summary_financial_status_project (
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("summary_financial_status_project".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data))
    }

    let get_projects = projects::Entity::find()
        .filter(projects::Column::IsDisposed.eq(false))
        .left_join(contractors::Entity)
        .left_join(settings_takers::Entity)
        .join_rev(sea_orm::JoinType::LeftJoin, projects_payment::Relation::Projects.def())
        .select_only()
        .select_column_as(projects::Column::ProjectName, "project_name")
        .select_column_as(projects::Column::ProjectCode, "project_code")
        .select_column_as(contractors::Column::Name, "contractor_name")
        .select_column_as(settings_takers::Column::Name, "taker_name")
        .select_column_as(projects::Column::ProjectStatus, "project_status")
        .select_column_as(projects::Column::StartDate, "project_start_date")
        .select_column_as(projects::Column::TargetDate, "project_target_date")
        .select_column_as(projects::Column::ContractCost, "project_contract_cost")
        .expr_as(Func::sum(Expr::case(
            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)).is_not_null(), Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid))).finally(Expr::value(0))), "project_paid")
        .group_by(projects::Column::ProjectCode)
        .group_by(projects::Column::ProjectName)
        .group_by(contractors::Column::Name)
        .group_by(settings_takers::Column::Name)
        .group_by(projects::Column::ProjectStatus)
        .group_by(projects::Column::StartDate)
        .group_by(projects::Column::TargetDate)
        .group_by(projects::Column::ContractCost)
        .into_model::<ResponseSummaryFinancialStatusProjects>()
        .all(&db.db_connection).await.unwrap();

    let total_contract_cost = get_projects.iter().fold(0, |n, acc| n + (acc.project_contract_cost.unwrap_or(0) as i64));
    let total_paid = get_projects.iter().fold(0, |n, acc| n + (acc.project_paid.unwrap_or(0) as i64));

    let make_response = ResponseSummaryFinancialStatusProjectsOverview {
        data: get_projects.clone(),
        total_records: get_projects.len(),
        total_contract_cost: total_contract_cost,
        total_paid: total_paid,
        total_balance: total_contract_cost - total_paid
    };

    redis.stored_value(&serde_json::to_string(&make_response).unwrap()).await.unwrap();
    

    (StatusCode::OK, Json(make_response))

}

pub async fn summary_financial_status_report (
    Extension(db): Extension<Arc<DB>>
) -> impl IntoResponse {

    let mut redis = Redis::new("summary_financial_status_report".to_string(), db.redis_connection.clone());

    if let Some(response) = redis.get_value().await {
        let json_data = serde_json::from_str(&response).unwrap();

        return (StatusCode::OK, Json(json_data))
    }

    let get_projects = try_join!(
        projects::Entity::find()
            .filter(Condition::all()
                .add(projects::Column::IsDisposed.eq(false))
                .add(projects::Column::ProjectStatus.ne("Not Yet Started"))
                .add(projects::Column::ProjectStatus.ne("Suspended"))
            )
            .join_rev(sea_orm::JoinType::LeftJoin, projects_payment::Relation::Projects.def())
            .select_only()
            .expr_as(Expr::col((projects::Entity, projects::Column::Id)).count(), "total_projects")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::Appropriation).is_not_null(), 
                            Expr::col(projects::Column::Appropriation))
                        .finally(Expr::value(0))), "total_appropriation")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0))), "total_contract_cost")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)).is_not_null(), 
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)))
                        .finally(Expr::value(0))), "total_paid")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0)))
                    .sub(Func::sum(
                    Expr::case(
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)).is_not_null(), 
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)))
                        .finally(Expr::value(0)))), "total_balance")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::Appropriation).is_not_null(), 
                            Expr::col(projects::Column::Appropriation))
                        .finally(Expr::value(0)))
                    .sub(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0)))), "total_savings")
            .into_model::<ResponseSummaryFinancialStatusReports>()
            .one(&db.db_connection),
        projects::Entity::find()
            .filter(Condition::all()
                .add(projects::Column::IsDisposed.eq(false))
                .add(projects::Column::ProjectStatus.eq("Not Yet Started"))
            )
            .join_rev(sea_orm::JoinType::LeftJoin, projects_payment::Relation::Projects.def())
            .select_only()
            .expr_as(Expr::col((projects::Entity, projects::Column::Id)).count(), "total_projects")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::Appropriation).is_not_null(), 
                            Expr::col(projects::Column::Appropriation))
                        .finally(Expr::value(0))), "total_appropriation")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0))), "total_contract_cost")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)).is_not_null(), 
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)))
                        .finally(Expr::value(0))), "total_paid")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0)))
                    .sub(Func::sum(
                    Expr::case(
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)).is_not_null(), 
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)))
                        .finally(Expr::value(0)))), "total_balance")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::Appropriation).is_not_null(), 
                            Expr::col(projects::Column::Appropriation))
                        .finally(Expr::value(0)))
                    .sub(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0)))), "total_savings")
            .into_model::<ResponseSummaryFinancialStatusReports>()
            .one(&db.db_connection),
        projects::Entity::find()
            .filter(Condition::all()
                .add(projects::Column::IsDisposed.eq(false))
                .add(projects::Column::ProjectStatus.eq("Suspended"))
            )
            .join_rev(sea_orm::JoinType::LeftJoin, projects_payment::Relation::Projects.def())
            .select_only()
            .expr_as(Expr::col((projects::Entity, projects::Column::Id)).count(), "total_projects")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::Appropriation).is_not_null(), 
                            Expr::col(projects::Column::Appropriation))
                        .finally(Expr::value(0))), "total_appropriation")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0))), "total_contract_cost")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)).is_not_null(), 
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)))
                        .finally(Expr::value(0))), "total_paid")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0)))
                    .sub(Func::sum(
                    Expr::case(
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)).is_not_null(), 
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)))
                        .finally(Expr::value(0)))), "total_balance")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::Appropriation).is_not_null(), 
                            Expr::col(projects::Column::Appropriation))
                        .finally(Expr::value(0)))
                    .sub(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0)))), "total_savings")
            .into_model::<ResponseSummaryFinancialStatusReports>()
            .one(&db.db_connection),
        projects::Entity::find()
            .filter(Condition::all()
                .add(projects::Column::IsDisposed.eq(false))
            )
            .join_rev(sea_orm::JoinType::LeftJoin, projects_payment::Relation::Projects.def())
            .select_only()
            .expr_as(Expr::col((projects::Entity, projects::Column::Id)).count(), "total_projects")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::Appropriation).is_not_null(), 
                            Expr::col(projects::Column::Appropriation))
                        .finally(Expr::value(0))), "total_appropriation")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0))), "total_contract_cost")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)).is_not_null(), 
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)))
                        .finally(Expr::value(0))), "total_paid")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0)))
                    .sub(Func::sum(
                    Expr::case(
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)).is_not_null(), 
                            Expr::col((projects_payment::Entity, projects_payment::Column::AmountPaid)))
                        .finally(Expr::value(0)))), "total_balance")
            .expr_as(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::Appropriation).is_not_null(), 
                            Expr::col(projects::Column::Appropriation))
                        .finally(Expr::value(0)))
                    .sub(Func::sum(
                    Expr::case(
                            Expr::col(projects::Column::ContractCost).is_not_null(), 
                            Expr::col(projects::Column::ContractCost))
                        .finally(Expr::value(0)))), "total_savings")
            .into_model::<ResponseSummaryFinancialStatusReports>()
            .one(&db.db_connection)
    ).unwrap();

    let (
        get_implemented_projects,
        get_unimplemented_projects,
        get_suspended_projects,
        get_all_projects
    ) = get_projects;


    let make_response = ResponseSummaryFinancialStatusReportsOverview {
        data_implemented: get_implemented_projects.unwrap(),
        data_unimplemented: get_unimplemented_projects.unwrap(),
        data_suspended: get_suspended_projects.unwrap(),
        data_summary: get_all_projects.unwrap()
    };

    redis.stored_value(&serde_json::to_string(&make_response).unwrap()).await.unwrap();

    (StatusCode::OK, Json(make_response))
}