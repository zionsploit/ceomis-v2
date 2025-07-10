use entity::user_roles;
use sea_orm_migration::{prelude::*, sea_orm::{ActiveValue, EntityTrait}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let db = manager.get_connection();
        
        // INITIALIZE BASE ROLES
        let bac_role = user_roles::ActiveModel {
            name: ActiveValue::set("Bac Office".to_string()),
            ..Default::default()
        };

        let engineer_role = user_roles::ActiveModel {
            name: ActiveValue::Set("Engineer's Office".to_string()),
            ..Default::default()
        };

        let mayor_role = user_roles::ActiveModel {
            name: ActiveValue::Set("Mayor's Office".to_string()),
            ..Default::default()
        };

        let planning_role = user_roles::ActiveModel {
            name: ActiveValue::Set("Planning Office".to_string()),
            ..Default::default()
        };

        user_roles::Entity::insert_many([bac_role, engineer_role, mayor_role, planning_role]).exec(db).await.unwrap();

        Ok(())
    }
}