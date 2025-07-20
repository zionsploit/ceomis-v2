use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.alter_table(TableAlterStatement::new()
            .table(ProjectsMonitoringImg::Table)
            .add_column_if_not_exists(string(ProjectsMonitoringImg::ImagesOriginalName))
            .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum ProjectsMonitoringImg {
    Table,
    ImagesOriginalName
}
