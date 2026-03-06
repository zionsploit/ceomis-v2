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
                id: Set(1)
            },
            settings_sof::ActiveModel {
                name: Set("20% Development Fund".to_string()),
                id: Set(2)
            },
            settings_sof::ActiveModel {
                name: Set("DRRMF".to_string()),
                id: Set(3)
            },
            settings_sof::ActiveModel {
                name: Set("GAD".to_string()),
                id: Set(4)
            },
            settings_sof::ActiveModel {
                name: Set("1SAIP".to_string()),
                id: Set(5)
            },
            settings_sof::ActiveModel {
                name: Set("2SAIP".to_string()),
                id: Set(6)
            },
            settings_sof::ActiveModel {
                name: Set("3SAIP".to_string()),
                id: Set(7)
            },
            settings_sof::ActiveModel {
                name: Set("4SAIP".to_string()),
                id: Set(8)
            },
            settings_sof::ActiveModel {
                name: Set("Local Government Support Fund".to_string()),
                id: Set(9)
            },
            settings_sof::ActiveModel {
                name: Set("Skill Council for Persons with Disabilities".to_string()),
                id: Set(10)
            },
            settings_sof::ActiveModel {
                name: Set("Special Education Fund".to_string()),
                id: Set(11)
            },
            settings_sof::ActiveModel {
                name: Set("5SAIP".to_string()),
                id: Set(12)
            },
            settings_sof::ActiveModel {
                name: Set("6SAIP".to_string()),
                id: Set(13)
            },
            settings_sof::ActiveModel {
                name: Set("7SAIP".to_string()),
                id: Set(14)
            },
            settings_sof::ActiveModel {
                name: Set("Relative Guardianship Assistance Program".to_string()),
                id: Set(15)
            },
            settings_sof::ActiveModel {
                name: Set("Local Councils for the Protection of Children".to_string()),
                id: Set(16)
            },
            settings_sof::ActiveModel {
                name: Set("SAILP".to_string()),
                id: Set(17)
            },
            settings_sof::ActiveModel {
                name: Set("Barangay Fund".to_string()),
                id: Set(18)
            },
            settings_sof::ActiveModel {
                name: Set("1SAIL23".to_string()),
                id: Set(19)
            },
            settings_sof::ActiveModel {
                name: Set("2SAILP21".to_string()),
                id: Set(20)
            },
            settings_sof::ActiveModel {
                name: Set("National Fund".to_string()),
                id: Set(21)
            },
            settings_sof::ActiveModel {
                name: Set("1SAIL22".to_string()),
                id: Set(22)
            },
            settings_sof::ActiveModel {
                name: Set("SEF".to_string()),
                id: Set(23)
            },
            settings_sof::ActiveModel {
                name: Set("SAIL".to_string()),
                id: Set(24)
            },
            settings_sof::ActiveModel {
                name: Set("2SAIL".to_string()),
                id: Set(25)
            },
            settings_sof::ActiveModel {
                name: Set("SIPO2-K17-DF-C3".to_string()),
                id: Set(26)
            },
            settings_sof::ActiveModel {
                name: Set("2SAIL20".to_string()),
                id: Set(27)
            },
        ];

        settings_sof::Entity::insert_many(make_add).exec(db).await.unwrap();

        Ok(())
    }
}
