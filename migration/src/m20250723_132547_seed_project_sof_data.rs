use entity::settings::settings_sof;
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();

        let make_add = [
            settings_sof::ActiveModel {
                name: Set("Regular Fund".to_string()),
                ..Default::default()
            },
            settings_sof::ActiveModel {
                name: Set("20% Development Fund".to_string()),
                ..Default::default()
            },
            settings_sof::ActiveModel {
                name: Set("DRRMF".to_string()),
                ..Default::default()
            },
            settings_sof::ActiveModel {
                name: Set("GAD".to_string()),
                ..Default::default()
            },
            settings_sof::ActiveModel {
                name: Set("1SAIP".to_string()),
                ..Default::default()
            },
            settings_sof::ActiveModel {
                name: Set("2SAIP".to_string()),
                ..Default::default()
            },
            settings_sof::ActiveModel {
                name: Set("3SAIP".to_string()),
                ..Default::default()
            },
            settings_sof::ActiveModel {
                name: Set("4SAIP".to_string()),
                ..Default::default()
            },
            settings_sof::ActiveModel {
                name: Set("Local Government Support Fund".to_string()),
                ..Default::default()
            },
            settings_sof::ActiveModel {
                name: Set("Skill Council for Persons with Disabilities".to_string()),
                ..Default::default()
            },
        ];

        settings_sof::Entity::insert_many(make_add).exec(db).await.unwrap();

        Ok(())
    }
}
