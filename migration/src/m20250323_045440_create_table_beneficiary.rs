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
                    .table(Beneficiary::Table)
                    .if_not_exists()
                    .col(pk_auto(Beneficiary::Id))
                    .col(string(Beneficiary::representant_name).not_null())
                    .col(string(Beneficiary::representant_lastname_f).not_null())
                    .col(string(Beneficiary::representant_lastname_m).not_null())
                    .col(string(Beneficiary::phone).not_null())
                    .col(string(Beneficiary::legal_name).not_null())
                    .col(string(Beneficiary::NIF).not_null().unique_key())
                    .col(date(Beneficiary::foundation_date).not_null())
                    .col(integer(Beneficiary::credentials_id).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-credentials-benefactor-id")
                            .from(Beneficiary::Table, Beneficiary::credentials_id)
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
            .drop_table(Table::drop().table(Beneficiary::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Beneficiary {
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
