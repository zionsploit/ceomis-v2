use entity::projects::ProjectStatus;
use sea_orm_migration::{prelude::{extension::postgres::TypeCreateStatement, *}, sea_orm::Schema};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let schema = Schema::new(manager.get_database_backend());

        let make_new_enum_table = schema.create_enum_from_active_enum::<ProjectStatus>();

        manager.create_type(TypeCreateStatement::from(make_new_enum_table).to_owned()).await
    }
}