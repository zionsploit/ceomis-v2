use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager.alter_table(
            Table::alter()
            .table(User::Table)
            .add_column_if_not_exists(
                ColumnDef::new(User::UserRolesId).integer()
            )
            .add_foreign_key(
                &TableForeignKey::new()
                    .name("fk-user-user_role_id-id")
                    .from_tbl(User::Table)
                    .from_col(User::UserRolesId)
                    .to_tbl(UserRoles::Table)
                    .to_col(UserRoles::Id)
            )
            .to_owned()
        ).await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    UserRolesId
}

#[derive(DeriveIden)]
enum UserRoles {
    Table,
    Id,
}
