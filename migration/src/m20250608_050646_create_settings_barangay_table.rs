use sea_orm_migration::{prelude::{extension::postgres::Type, *}, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.create_type(
            Type::create()
                .as_enum(Alias::new("barangay_type"))
                .values([Alias::new("Rural"), Alias::new("Urban")])
                .to_owned()
        ).await.unwrap();
        
        manager
            .create_table(
                Table::create()
                    .table(SettingsBarangay::Table)
                    .if_not_exists()
                    .col(pk_auto(SettingsBarangay::Id))
                    .col(string(SettingsBarangay::Name))
                    .col(ColumnDef::new(SettingsBarangay::BarangayType)
                        .enumeration(Alias::new("barangay_type"), [Alias::new("Rural"), Alias::new("Urban")]))
                    .col(boolean(SettingsBarangay::IsPoblacion))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(SettingsBarangay::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SettingsBarangay {
    Table,
    Id,
    Name,
    BarangayType,
    IsPoblacion
}
