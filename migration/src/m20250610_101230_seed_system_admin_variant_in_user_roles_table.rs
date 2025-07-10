use entity::user_roles;
use sea_orm_migration::{prelude::*, sea_orm::{ActiveValue, EntityTrait}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        
        let db = manager.get_connection();

        let sysadmin_role = user_roles::ActiveModel {
            name: ActiveValue::Set("System Admin".to_string()),
            ..Default::default()
        };

        user_roles::Entity::insert(sysadmin_role).exec(db).await.unwrap();

        Ok(())
    }
}
