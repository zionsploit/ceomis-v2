use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(UserInfo::Table)
                    .if_not_exists()
                    .col(pk_auto(UserInfo::Id))
                    .col(string(UserInfo::FirstName))
                    .col(string(UserInfo::MiddleName))
                    .col(string(UserInfo::LastName))
                    .col(integer(UserInfo::UserId).unique_key())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk-user-user_info_id-id")
                            .from_tbl(UserInfo::Table)
                            .from_col(UserInfo::UserId)
                            .to_tbl(User::Table)
                            .to_col(User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(UserInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserInfo {
    Table,
    Id,
    FirstName,
    MiddleName,
    LastName,
    UserId
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id
}