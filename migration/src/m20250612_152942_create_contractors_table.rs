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
                    .table(Contractors::Table)
                    .if_not_exists()
                    .col(pk_auto(Contractors::Id))
                    .col(string(Contractors::Name))
                    .col(string(Contractors::EmailAddress))
                    .col(string(Contractors::AddressStreet))
                    .col(string(Contractors::AddressBarangay))
                    .col(string(Contractors::AddressMunicipality))
                    .col(string(Contractors::AddressProvince))
                    .col(string_null(Contractors::About))
                    .col(string(Contractors::ContactFullName))
                    .col(string(Contractors::ContactPosition))
                    .col(string(Contractors::ContactNumber))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Contractors::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Contractors {
    Table,
    Id,
    Name,
    EmailAddress,
    AddressStreet,
    AddressBarangay,
    AddressMunicipality,
    AddressProvince,
    About,
    ContactFullName,
    ContactPosition,
    ContactNumber,
}
