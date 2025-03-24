use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .create_table(
                Table::create()
                    .table(Donator::Table)
                    .if_not_exists()
                    .col(pk_auto(Donator::Id))
                    .col(string(Donator::name).not_null())
                    .col(string(Donator::lastname_f).not_null())
                    .col(string(Donator::lastname_m).not_null())
                    .col(string(Donator::phone).not_null())
                    .col(string(Donator::organization_name).not_null())
                    .col(integer(Donator::credentials_id).not_null())
                    .foreign_key(
                            ForeignKey::create()
                                .name("fk-credentials-donator-id")
                                .from(Donator::Table, Donator::credentials_id)
                                .to(super::m20250323_044314_create_table_credentials::Credentials::Table, super::m20250323_044314_create_table_credentials::Credentials::Id)
                                .on_delete(ForeignKeyAction::Cascade)
                                .on_update(ForeignKeyAction::Cascade)
                    )



                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Donator::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Donator {
    Table,
    Id,
    name,
    lastname_f,
    lastname_m,
    phone,
    organization_name,
    credentials_id
}
