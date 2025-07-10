use entity::projects;
use sea_orm_migration::{prelude::*, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        
        let schema = Schema::new(manager.get_database_backend());

        let make_project_table = schema.create_table_from_entity(projects::Entity);

        manager.create_table(make_project_table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(
            TableDropStatement::new()
                .table(Projects::Table)
                .to_owned() 
        ).await
    }
}

#[derive(DeriveIden)]
pub enum Projects {
    Table
}