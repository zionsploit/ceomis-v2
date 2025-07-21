use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.alter_table(TableAlterStatement::new()
            .table(ProjectsMonitoringRemarks::Table)
            .add_column_if_not_exists(integer(ProjectsMonitoringRemarks::ProjectId))
            .add_foreign_key(
                &TableForeignKey::new()
                    .name("fk-projects_monitoring_remarks-projects_id-id")
                    .from_tbl(ProjectsMonitoringRemarks::Table)
                    .from_col(ProjectsMonitoringRemarks::ProjectId)
                    .to_tbl(Projects::Table)
                    .to_col(Projects::Id)   
            )
            .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id
}


#[derive(DeriveIden)]
enum ProjectsMonitoringRemarks {
    Table,
    ProjectId
}
