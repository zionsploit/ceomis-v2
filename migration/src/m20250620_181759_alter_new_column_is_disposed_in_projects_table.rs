use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        
        manager.alter_table(
            TableAlterStatement::new()
                .table(Projects::Table)
                .add_column(boolean(Projects::IsDisposed).default(false))
                .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
pub enum Projects {
    Table,
    IsDisposed
}