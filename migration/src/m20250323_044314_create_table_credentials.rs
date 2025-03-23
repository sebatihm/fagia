use sea_orm_migration::{prelude::*, schema::*, sea_orm::{EnumIter, Iterable}};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(Credentials::Table)
                    .if_not_exists()
                    .col(pk_auto(Credentials::Id))
                    .col(string(Credentials::email).not_null().unique_key())
                    .col(string(Credentials::password).not_null())
                    .col(enumeration(Credentials::r#type, Alias::new("type"), r#type::iter()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Credentials::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Credentials {
    Table,
    Id,
    email,
    password,
    r#type
}

#[derive(Iden, EnumIter)]
pub enum r#type {
    DONATOR,
    BENEFACTOR
}