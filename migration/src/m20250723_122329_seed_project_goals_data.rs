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
                ..Default::default()
            },
            settings_sdg::ActiveModel {
                name: Set("Zero Hunger".to_string()),
                ..Default::default()
            },
            settings_sdg::ActiveModel {
                name: Set("Good Health and Well-Being".to_string()),
                ..Default::default()
            },
            settings_sdg::ActiveModel {
                name: Set("Quality Education".to_string()),
                ..Default::default()
            },
            settings_sdg::ActiveModel {
                name: Set("Gender Equality".to_string()),
                ..Default::default()
            },
            settings_sdg::ActiveModel {
                name: Set("Clean Water and Sanitation".to_string()),
                ..Default::default()
            },
            settings_sdg::ActiveModel {
                name: Set("Affordable and Clean Energy".to_string()),
                ..Default::default()
            },
            settings_sdg::ActiveModel {
                name: Set("Decent Work and Economic Growth".to_string()),
                ..Default::default()
            },
            settings_sdg::ActiveModel {
                name: Set("Industry, Innovation and Infrastructure".to_string()),
                ..Default::default()
            },
            settings_sdg::ActiveModel {
                name: Set("Reduced Inequalities".to_string()),
                ..Default::default()
            },
       ];

       settings_sdg::Entity::insert_many(make_projects_goals_data).exec(db).await.unwrap();

       Ok(())
    }
}
