use entity::{contractors, projects::{ActiveModel, ProjectStatus}, settings::settings_takers};
use sea_orm::{ActiveValue::Set, EntityTrait, QueryFilter, prelude::*};
use sea_orm_migration::SchemaManagerConnection;
use serde_json::Value;

pub fn convert_projects_into_active_model(
    json: &Value, 
    contractor_id: Option<i32>, 
    project_takers_id: Option<i32>
) -> ActiveModel {
    let attributes = &json["attributes"];

    ActiveModel {
        project_year: Set(attributes["year"].as_i64().map(|v| v as i32).unwrap()),
        project_name: Set(attributes["name"].as_str().map(|v| v.to_string()).unwrap()),
        project_code: Set(attributes["code"].as_str().map(|v| v.to_string()).unwrap()),
        project_status: {
            let status_str = attributes["project_status"].as_str().unwrap_or("");
            Set(parse_project_status(status_str))
        },
        barangays: {
            let locations = attributes["location"].as_str().unwrap_or("[]");
            let parsed: Vec<i32> = serde_json::from_str(locations).unwrap_or_default();
            Set(Some(parsed))
        },
        appropriation: Set(attributes["appropriation"].as_f64().map(|v| v as i32)),
        approved_budget_contact: {
            let abc = attributes["abc"].as_f64().unwrap_or(0.0) as i32;
            if abc > 0 { Set(Some(abc)) } else { Set(None) }
        },
        contractor_id: Set(contractor_id),
        contract_cost: Set(Some(attributes["cost"].as_f64().unwrap_or(0.0) as i32)),
        start_date: {
            let start = attributes["start"].as_str().unwrap_or("").trim();
            if start.is_empty() { Set(None) } else { Set(Some(start.to_string())) }
        },
        calendar_days: Set(None), // duration skipped
        time_extensions: {
            let ext = attributes["time_extension"].as_i64().unwrap_or(0);
            if ext > 0 { Set(Some(ext as i32)) } else { Set(None) }
        },
        target_date: {
            let target = attributes["target"].as_str().unwrap_or("").trim();
            if target.is_empty() { Set(None) } else { Set(Some(target.to_string())) }
        },
        project_type_id: {
            let prop_type = attributes["prop_type"].as_str().and_then(|v| v.parse::<i32>().ok());
            Set(prop_type)
        },
        project_category_id: {
            let prop_category = attributes["prop_category"].as_str().and_then(|v| v.parse::<i32>().ok());
            Set(prop_category)
        },
        project_sof_id: {
            let prop_fund = attributes["prop_fund"].as_str().and_then(|v| v.parse::<i32>().ok());
            Set(prop_fund)
        },
        project_incharge_id: {
            let prop_assign = attributes["prop_assign"].as_str().and_then(|v| v.parse::<i32>().ok());
            Set(prop_assign)
        },
        sustainable_development_goals: {
            let sdg = attributes["prop_sdg"].as_str().unwrap_or("[]");
            let parsed: Vec<i32> = serde_json::from_str(sdg).unwrap_or_default();
            if parsed.is_empty() { Set(None) } else { Set(Some(parsed)) }
        },
        sector: {
            let sector = attributes["prop_sector"].as_str().unwrap_or("[]");
            let parsed: Vec<i32> = serde_json::from_str(sector).unwrap_or_default();
            if parsed.is_empty() { Set(None) } else { Set(Some(parsed)) }
        },
        project_takers_id: Set(project_takers_id),
        accomplished: Set(Some(attributes["accom_total"].as_i64().unwrap_or(0) as i16)),
        remarks: {
            let remarks = attributes["remarks"].as_str().unwrap_or("").trim();
            if remarks.is_empty() { Set(None) } else { Set(Some(remarks.to_string())) }
        },
        prepared_users_id: Set(None),
        ..Default::default()
    }
}

fn parse_project_status(value: &str) -> Option<ProjectStatus> {
    match value.trim() {
        "Not Yet Started" => Some(ProjectStatus::NotYetStarted),
        "Preparation" => Some(ProjectStatus::Preparation),
        "Bidding" => Some(ProjectStatus::Bidding),
        "Bidded" => Some(ProjectStatus::Bidded),
        "On-Going" => Some(ProjectStatus::OnGoing),
        "Completed" => Some(ProjectStatus::Completed),
        "Suspended" => Some(ProjectStatus::Suspended),
        _ => None
    }
}

pub async fn convert_projects_into_active_model_v2<'c>(
    json: &Value,
    db_connection: &SchemaManagerConnection<'c>
) -> ActiveModel {
    let attributes = &json["attributes"];

    ActiveModel {
        project_year: Set(attributes["year"].as_i64().map(|v| v as i32).unwrap()),
        project_name: Set(attributes["name"].as_str().map(|v| v.to_string()).unwrap()),
        project_code: Set(attributes["code"].as_str().map(|v| v.to_string()).unwrap()),
        project_status: {
            let status_str = attributes["project_status"].as_str().unwrap_or("");
            Set(parse_project_status(status_str))
        },
        barangays: {
            let locations = attributes["location"].as_str().unwrap_or("[]");
            let parsed: Vec<i32> = serde_json::from_str(locations).unwrap_or_default();
            Set(Some(parsed))
        },
        appropriation: Set(attributes["appropriation"].as_f64().map(|v| v as i32)),
        approved_budget_contact: {
            let abc = attributes["abc"].as_f64().unwrap_or(0.0) as i32;
            if abc > 0 { Set(Some(abc)) } else { Set(None) }
        },
        contractor_id: {
            let get_contractor_name = attributes
                .get("contractor")
                .and_then(|v| v.get("data"))
                .and_then(|v| v.get("attributes"))
                .and_then(|v | v.get("name"))
                .and_then(|v| v.as_str());

            if get_contractor_name.is_none() {
                Set(None)
            } else {
                let find_contract_id = contractors::Entity::find()
                    .filter(contractors::Column::Name.contains(get_contractor_name.unwrap())).one(db_connection).await.unwrap();

                Set(Some(find_contract_id.unwrap().id))
            } 
        },
        contract_cost: Set(Some(attributes["cost"].as_f64().unwrap_or(0.0) as i32)),
        start_date: {
            let start = attributes["start"].as_str().unwrap_or("").trim();
            if start.is_empty() { Set(None) } else { Set(Some(start.to_string())) }
        },
        calendar_days: Set(None), // duration skipped
        time_extensions: {
            let ext = attributes["time_extension"].as_i64().unwrap_or(0);
            if ext > 0 { Set(Some(ext as i32)) } else { Set(None) }
        },
        target_date: {
            let target = attributes["target"].as_str().unwrap_or("").trim();
            if target.is_empty() { Set(None) } else { Set(Some(target.to_string())) }
        },
        project_type_id: {
            let prop_type = attributes["prop_type"].as_str().and_then(|v| v.parse::<i32>().ok());

            if prop_type.is_some_and(|v| v.gt(&10)) {
                Set(None)
            } else {
                Set(prop_type)
            }
        },
        project_category_id: {
            let prop_category = attributes["prop_category"].as_str().and_then(|v| v.parse::<i32>().ok());
            Set(prop_category)
        },
        project_sof_id: {
            let prop_fund = attributes["prop_fund"].as_str().and_then(|v| v.parse::<i32>().ok());

            if prop_fund.is_some_and(|v| v.gt(&10)) {
                Set(None)
            } else {
                Set(prop_fund)
            }
        },
        project_incharge_id: {
            let prop_assign = attributes["prop_assign"].as_str().and_then(|v| v.parse::<i32>().ok());
            Set(prop_assign)
        },
        sustainable_development_goals: {
            let sdg = attributes["prop_sdg"].as_str().unwrap_or("[]");
            let parsed: Vec<i32> = serde_json::from_str(sdg).unwrap_or_default();
            if parsed.is_empty() { Set(None) } else { Set(Some(parsed)) }
        },
        sector: {
            let sector = attributes["prop_sector"].as_str().unwrap_or("[]");
            let parsed: Vec<i32> = serde_json::from_str(sector).unwrap_or_default();
            if parsed.is_empty() { Set(None) } else { Set(Some(parsed)) }
        },
        project_takers_id: {
            let get_project_takers = attributes.get("prop_takers").and_then(|v| v.as_str());

            if get_project_takers.is_none() {
                Set(None)
            } else {
                let const_parse_project_name = get_project_takers.unwrap().trim();

                let find_contract_id = settings_takers::Entity::find()
                    .filter(settings_takers::Column::Name.contains(const_parse_project_name)).one(db_connection).await.unwrap();

                if find_contract_id.is_none() {
                    Set(None)
                } else {
                    Set(Some(find_contract_id.unwrap().id))
                }
            }

        },
        accomplished: Set(Some(attributes["accom_total"].as_i64().unwrap_or(0) as i16)),
        remarks: {
            let remarks = attributes["remarks"].as_str().unwrap_or("").trim();
            if remarks.is_empty() { Set(None) } else { Set(Some(remarks.to_string())) }
        },
        prepared_users_id: Set(None),
        ..Default::default()
    }
}
