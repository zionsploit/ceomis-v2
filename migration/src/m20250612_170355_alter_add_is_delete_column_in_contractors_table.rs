use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
           .alter_table(
            TableAlterStatement::new()
                .table(Contractors::Table)
                .add_column(boolean(Contractors::IsDelete).default(false))
                .to_owned()
           )
            .await
    }
}

#[derive(DeriveIden)]
enum Contractors {
    Table,
    IsDelete
}
