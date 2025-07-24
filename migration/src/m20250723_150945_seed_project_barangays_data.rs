use entity::settings::settings_barangay;
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
            settings_barangay::ActiveModel {
                name: Set("Balangasan".to_string()),
                is_poblacion: Set(true),
                barangay_type: Set(Some(settings_barangay::BarangayType::Rural)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Balintawak".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Banale".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Buenavista".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Dao".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Dumagoc".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Gatas".to_string()),
                is_poblacion: Set(true),
                barangay_type: Set(Some(settings_barangay::BarangayType::Rural)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Kawit".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Lumbia".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Napolan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("San Francisco".to_string()),
                is_poblacion: Set(true),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("San Jose".to_string()),
                is_poblacion: Set(true),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("San Pedro".to_string()),
                is_poblacion: Set(true),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Santa Lucia".to_string()),
                is_poblacion: Set(true),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Santa Maria".to_string()),
                is_poblacion: Set(true),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Santiago".to_string()),
                is_poblacion: Set(true),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Santo Niño".to_string()),
                is_poblacion: Set(true),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Tiguma".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Tuburan".to_string()),
                is_poblacion: Set(true),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Alegria".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Baloyboan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Bogo".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Bomba".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Bulatok".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Bulawan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Dampalan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Danlugan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Datagan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Deborok".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Ditoray".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Gubac".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Gubang".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Kagawasan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Kahayagan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Kalasan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("La Suerte".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Lala".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Lapidian".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Lenienza".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Lison Valley".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Lourdes".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Lower Sibatang".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Lumad".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Macasing".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Manga".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Muricay".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Palpalan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Pedulonan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Poloyagan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Tawagan Sur".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Tulangan".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Tulawas".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("Upper Sibatang".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
            settings_barangay::ActiveModel {
                name: Set("White Beach".to_string()),
                is_poblacion: Set(false),
                barangay_type: Set(Some(settings_barangay::BarangayType::Urban)),
                ..Default::default()
            },
        ];

        settings_barangay::Entity::insert_many(make_add).exec(db).await.unwrap();

        Ok(())
    }
}