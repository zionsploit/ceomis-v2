use entity::settings::settings_type;
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
            settings_type::ActiveModel {
                name: Set("Local Road-Road Concreting".to_string()),
                id: Set(1)
            },
            settings_type::ActiveModel {
                name: Set("Bridge".to_string()),
                id: Set(2)
            },
            settings_type::ActiveModel {
                name: Set("Public Market".to_string()),
                id: Set(3)
            },
            settings_type::ActiveModel {
                name: Set("Slaughterhouse".to_string()),
                id: Set(4)
            },
            settings_type::ActiveModel {
                name: Set("Multi-purpose Building/Hall".to_string()),
                id: Set(5)
            },
            settings_type::ActiveModel {
                name: Set("Multi-purpose Pavement".to_string()),
                id: Set(6)
            },
            settings_type::ActiveModel {
                name: Set("Drainage Canal".to_string()),
                id: Set(7)
            },
            settings_type::ActiveModel {
                name: Set("Sea Wall/River Control".to_string()),
                id: Set(8)
            },
            settings_type::ActiveModel {
                name: Set("Water System".to_string()),
                id: Set(9)
            },
            settings_type::ActiveModel {
                name: Set("Evacuation Center".to_string()),
                id: Set(10)
            },
            settings_type::ActiveModel {
                name: Set("Public Park".to_string()),
                id: Set(11)
            },
            settings_type::ActiveModel {
                name: Set("Fish Port".to_string()),
                id: Set(12)
            },
            settings_type::ActiveModel {
                name: Set("Post-harvest facilities composed of ice plant and cold storage facility".to_string()),
                id: Set(13)
            },
            settings_type::ActiveModel {
                name: Set("Police Outpost".to_string()),
                id: Set(14)
            },
            settings_type::ActiveModel {
                name: Set("Health Center".to_string()),
                id: Set(15)
            },
            settings_type::ActiveModel {
                name: Set("Covered Court".to_string()),
                id: Set(16)
            },
            settings_type::ActiveModel {
                name: Set("Barangay Hall".to_string()),
                id: Set(17)
            },
            settings_type::ActiveModel {
                name: Set("Gymnasium".to_string()),
                id: Set(18)
            },
            settings_type::ActiveModel {
                name: Set("Day Care Center".to_string()),
                id: Set(19)
            },
            settings_type::ActiveModel {
                name: Set("Stage".to_string()),
                id: Set(20)
            },
            settings_type::ActiveModel {
                name: Set("Purok Hall".to_string()),
                id: Set(21)
            },
            settings_type::ActiveModel {
                name: Set("Spillway".to_string()),
                id: Set(22)
            },
            settings_type::ActiveModel {
                name: Set("Box Culvert".to_string()),
                id: Set(23)
            },
            settings_type::ActiveModel {
                name: Set("School building".to_string()),
                id: Set(24)
            },
            settings_type::ActiveModel {
                name: Set("Welcome Arch".to_string()),
                id: Set(25)
            },
            settings_type::ActiveModel {
                name: Set("Building".to_string()),
                id: Set(26)
            },
            settings_type::ActiveModel {
                name: Set("Perimeter  Fence".to_string()),
                id: Set(27)
            },
            settings_type::ActiveModel {
                name: Set("Perimeter  Fence".to_string()),
                id: Set(28)
            },
            settings_type::ActiveModel {
                name: Set("Local Road-Road Asphalting".to_string()),
                id: Set(29)
            },
            settings_type::ActiveModel {
                name: Set("Local Road-Drainage".to_string()),
                id: Set(30)
            },
            settings_type::ActiveModel {
                name: Set("Local Road- Bridges".to_string()),
                id: Set(31)
            },
            settings_type::ActiveModel {
                name: Set("Local Road -Widening".to_string()),
                id: Set(32)
            },
            settings_type::ActiveModel {
                name: Set("Local Road-Gravelling".to_string()),
                id: Set(33)
            },
       ];

        settings_type::Entity::insert_many(make_add).exec(db).await.unwrap();

       Ok(())
    }
}