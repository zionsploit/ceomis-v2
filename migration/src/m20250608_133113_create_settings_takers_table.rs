use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(SettingsTakers::Table)
                    .if_not_exists()
                    .col(pk_auto(SettingsTakers::Id))
                    .col(string(SettingsTakers::Name))
                    .col(string(SettingsTakers::ContactNumber))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(SettingsTakers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SettingsTakers {
    Table,
    Id,
    Name,
    ContactNumber,
}
