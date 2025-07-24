use entity::settings::settings_takers;
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
            settings_takers::ActiveModel {
                name: Set("ENGR. RELAMPAGOS".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("LANCE CO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("JIMWEN CONSTRUCTION".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("EDWIN DY & PATRICK WEE".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("ENGR. PAMERON".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("JOMARC & FERDIE".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("ERIC BERSALED".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("KAGAWAD ALBARACIN".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("COUN CAGAMPANG".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("YEN-YEN BONTILAO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("SADAM DIMASAR".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("LYNETTE PERALTA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. DACAL".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("PARAGUYA REMIL".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CTG-HERME GUINEA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. BALUDO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("ANGGA DABLO".to_string()),
                contact_number: Set(Some("09176257115".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("ARIEL LIM".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CHITO ALFORQUE".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("DODONG CAGAMPANG".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("BOLOTAOLO REVELO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("TENG MARCABAN".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. POLOYAGAN EDGAR REVELO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CORPO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CHAMPION IPIL".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("RD VINCE-DOLE".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("RIC ALAJENO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. MAGO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("BING BORBON GARRY HINOGUIN".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("TOTO LU".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("GLAVYS GAVENIA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CECIL TURA".to_string()),
                contact_number: Set(Some("09398245675".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("SUSAN BEARMINO BONTILAO".to_string()),
                contact_number: Set(Some("09505651740".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("KAG. ORDENIZA-BOGO".to_string()),
                contact_number: Set(Some("09487176107".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("KAG.VEDRA/ANGGO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("KAG. VEDRA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("AU-AU BASAS".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("EMELYE ARANAS".to_string()),
                contact_number: Set(Some("0999991309".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("JABO (LANCE CO)".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("ARIES MADARANG".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("VIC LINGATING".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. SUMAMPONG".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. DIMASAR".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("KAG. MURICAY".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("KAG. EDGAR REVELO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. INGKIT".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("YENYEN BONTILAO".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CHAMPION IPIL- ELENA".to_string()),
                contact_number: Set(Some("09278305516".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. DENOPOL".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. BABANTA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. BUSTAMANTE".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("JOEL HOFILENA".to_string()),
                contact_number: Set(Some("09491215525".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. MANGOY KAG TEJADA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("JOMARC".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("PHILIP DURAN".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("rsq".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("LSC".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("Genevive Malalis".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("VIC2X CANETE".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("MARIANETTE MARCABAN".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CAPT. ESMAEL".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("FLORNINA GERONA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("HERMINIA GUINEA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("Jun Mondarte".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("CRISTY FERNANDEZ - CEBUPAC".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("Oliver Santos".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("King Macaampao".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("Dodong Pamaran".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("JIGGER ARIOSA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("DAENA PAMARAN".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("NELY SISONA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("MARC QUIMADA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("IRENE".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                name: Set("KAG CALMA".to_string()),
                contact_number: Set(Some("".to_string())),
                ..Default::default()
            },
            settings_takers::ActiveModel {
                 name: Set("STAR ARAO".to_string()),
                 contact_number: Set(Some("".to_string())),
                 ..Default::default()
            },
            settings_takers::ActiveModel {
                 name: Set("CAPT. ARAO-ARAO".to_string()),
                 contact_number: Set(Some("".to_string())),
                 ..Default::default()
            },
            settings_takers::ActiveModel {
                 name: Set("ARNNE IGANO".to_string()),
                 contact_number: Set(Some("".to_string())),
                 ..Default::default()
            },
        ];

        settings_takers::Entity::insert_many(make_add).exec(db).await.unwrap();

        Ok(())
    }
}