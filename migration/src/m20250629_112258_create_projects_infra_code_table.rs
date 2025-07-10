use entity::projects_infra_code;
use sea_orm::Schema;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let schema = Schema::new(manager.get_database_backend());

        let make_projects_infra_code = schema.create_table_from_entity(projects_infra_code::Entity);

        manager.create_table(make_projects_infra_code).await
    }
}
