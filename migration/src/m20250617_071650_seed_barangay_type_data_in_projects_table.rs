use entity::projects::{self, ProjectStatus};
use sea_orm_migration::{prelude::*, sea_orm::{ActiveValue::Set, EntityTrait}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();

        let projects_data = vec![
            projects::ActiveModel { 
                project_year: Set(2020),
                project_name: Set("Improvement of Multi-purpose Hall, Brgy. Tiguma".to_string()),
                project_code: Set("R-71-S-2020".to_string()),
                project_status: Set(Some(ProjectStatus::Preparation)),
                barangays: Set(Some(vec![18])),
                appropriation: Set(Some(100000)),
                project_type_id: Set(Some(5)),
                project_category_id: Set(Some(4)),
                project_sof_id: Set(Some(18)),
                prepared_users_id: Set(Some(4)),
                ..Default::default()
            },
            projects::ActiveModel { 
                project_year: Set(2023),
                project_name: Set("Improvement of Water System at Brgy. Lison Valley".to_string()),
                project_code: Set("RS#001-/202316".to_string()),
                project_status: Set(Some(ProjectStatus::Completed)),
                barangays: Set(Some(vec![19])),
                appropriation: Set(Some(300000)),
                project_type_id: Set(Some(5)),
                project_category_id: Set(Some(2)),
                project_sof_id: Set(Some(18)),
                project_incharge_id: Set(Some(3)),
                prepared_users_id: Set(Some(4)),
                ..Default::default()
            },
            projects::ActiveModel { 
                project_year: Set(2023),
                project_name: Set("Conc of Road Towards Prk Daisy to Cadena de Amor Leading to Kagawasan (1/2 Lane), Barangay Alegria".to_string()),
                project_code: Set("Barangay Fund".to_string()),
                project_status: Set(Some(ProjectStatus::OnGoing)),
                barangays: Set(Some(vec![19])),
                appropriation: Set(Some(300000)),
                project_type_id: Set(Some(1)),
                project_category_id: Set(Some(2)),
                project_sof_id: Set(Some(18)),
                project_incharge_id: Set(Some(2)),
                prepared_users_id: Set(Some(4)),
                ..Default::default()
            },
            projects::ActiveModel { 
                project_year: Set(2023),
                project_name: Set("Road Concreting of Alley at Purok Lower LOmboy, Brgy. Tuburan".to_string()),
                project_code: Set("Barangay Fund".to_string()),
                project_status: Set(Some(ProjectStatus::NotYetStarted)),
                project_type_id: Set(Some(1)),
                project_category_id: Set(Some(2)),
                project_sof_id: Set(Some(18)),
                project_incharge_id: Set(Some(2)),
                prepared_users_id: Set(Some(4)),
                ..Default::default()
            },
            projects::ActiveModel { 
                project_year: Set(2023),
                project_name: Set("Road Concreting of Alley @ Purok Nangka at Barangay Tuburan".to_string()),
                project_code: Set("Barangay Fund".to_string()),
                project_status: Set(Some(ProjectStatus::Completed)),
                barangays: Set(Some(vec![19])),
                project_type_id: Set(Some(1)),
                project_category_id: Set(Some(2)),
                project_sof_id: Set(Some(18)),
                project_incharge_id: Set(Some(2)),
                prepared_users_id: Set(Some(4)),
                ..Default::default()
            },
            projects::ActiveModel { 
                project_year: Set(2023),
                project_name: Set("Construction of Open Drainage @ Purok Marang, Brgy. Tuburan".to_string()),
                project_code: Set("Barangay Fund".to_string()),
                project_status: Set(Some(ProjectStatus::Completed)),
                barangays: Set(Some(vec![18])),
                project_type_id: Set(Some(7)),
                project_category_id: Set(Some(1)),
                project_sof_id: Set(Some(18)),
                project_incharge_id: Set(Some(2)),
                prepared_users_id: Set(Some(4)),
                ..Default::default()
            }
        ];

        projects::Entity::insert_many(projects_data).exec(db).await.unwrap();

        Ok(())
    }
}