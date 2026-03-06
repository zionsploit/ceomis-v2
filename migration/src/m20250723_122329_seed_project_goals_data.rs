use entity::settings::settings_sdg;
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
       let db = manager.get_connection();

       let make_projects_goals_data = [
            settings_sdg::ActiveModel {
                name: Set("No Poverty".to_string()),
                id: Set(1)
            },
            settings_sdg::ActiveModel {
                name: Set("Zero Hunger".to_string()),
                id: Set(2)
            },
            settings_sdg::ActiveModel {
                name: Set("Good Health and Well-Being".to_string()),
                id: Set(3)
            },
            settings_sdg::ActiveModel {
                name: Set("Quality Education".to_string()),
                id: Set(4)
            },
            settings_sdg::ActiveModel {
                name: Set("Gender Equality".to_string()),
                id: Set(5)
            },
            settings_sdg::ActiveModel {
                name: Set("Clean Water and Sanitation".to_string()),
                id: Set(6)
            },
            settings_sdg::ActiveModel {
                name: Set("Affordable and Clean Energy".to_string()),
                id: Set(7)
            },
            settings_sdg::ActiveModel {
                name: Set("Decent Work and Economic Growth".to_string()),
                id: Set(8)
            },
            settings_sdg::ActiveModel {
                name: Set("Industry, Innovation and Infrastructure".to_string()),
                id: Set(9)
            },
            settings_sdg::ActiveModel {
                name: Set("Reduced Inequalities".to_string()),
                id: Set(10)
            },
            settings_sdg::ActiveModel {
                name: Set("Sustainable Cities and Communities".to_string()),
                id: Set(11)
            },
            settings_sdg::ActiveModel {
                name: Set("Responsible Consumption and Production".to_string()),
                id: Set(12)
            },
            settings_sdg::ActiveModel {
                name: Set("Life Below Water".to_string()),
                id: Set(13)
            },
            settings_sdg::ActiveModel {
                name: Set("Climate Action".to_string()),
                id: Set(14)
            },
            settings_sdg::ActiveModel {
                name: Set("Life on Land".to_string()),
                id: Set(15)
            },
            settings_sdg::ActiveModel {
                name: Set("Peace, Justice and Strong Institutions".to_string()),
                id: Set(16)
            },
            settings_sdg::ActiveModel {
                name: Set("Partnerships for the Goals".to_string()),
                id: Set(17)
            },
       ];

       settings_sdg::Entity::insert_many(make_projects_goals_data).exec(db).await.unwrap();

       Ok(())
    }
}
