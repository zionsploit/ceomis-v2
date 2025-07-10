use entity::projects_payment;
use sea_orm::Schema;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        
        let schema = Schema::new(manager.get_database_backend());

        let make_table = schema.create_table_from_entity(projects_payment::Entity);

        manager.create_table(make_table).await
    }
}