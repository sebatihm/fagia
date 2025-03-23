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
                    .table(Benefactor::Table)
                    .if_not_exists()
                    .col(pk_auto(Benefactor::Id))
                    .col(string(Benefactor::representant_name).not_null())
                    .col(string(Benefactor::representant_lastname_f).not_null())
                    .col(string(Benefactor::representant_lastname_m).not_null())
                    .col(string(Benefactor::phone).not_null())
                    .col(string(Benefactor::legal_name).not_null())
                    .col(string(Benefactor::NIF).not_null().unique_key())
                    .col(date(Benefactor::foundation_date).not_null())
                    .col(string(Benefactor::credentials_id).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-credentials-donator-id")
                            .from(Benefactor::Table, Benefactor::credentials_id)
                            .to(super::m20250323_044314_create_table_credentials::Credentials::Table, super::m20250323_044314_create_table_credentials::Credentials::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(Benefactor::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Benefactor {
    Table,
    Id,
    representant_name,
    representant_lastname_f,
    representant_lastname_m,
    phone,
    legal_name,
    NIF,
    foundation_date,
    credentials_id
}
