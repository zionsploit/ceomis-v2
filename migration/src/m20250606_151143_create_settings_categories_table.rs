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
                    .table(SettingsCategories::Table)
                    .if_not_exists()
                    .col(pk_auto(SettingsCategories::Id))
                    .col(string(SettingsCategories::Name))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(SettingsCategories::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SettingsCategories {
    Table,
    Id,
    Name,
}
