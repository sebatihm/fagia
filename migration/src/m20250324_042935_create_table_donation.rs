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
                    .table(Donation::Table)
                    .if_not_exists()
                    .col(pk_auto(Donation::Id))
                    .col(date(Donation::date).not_null())
                    .col(integer(Donation::id_beneficiary).not_null())
                    .col(integer(Donation::id_donator).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-donation-beneficiary")
                            .from(Donation::Table, Donation::id_beneficiary)
                            .to(super::m20250323_045440_create_table_beneficiary::Beneficiary::Table, super::m20250323_045440_create_table_beneficiary::Beneficiary::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)   
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-donation-donator")
                            .from(Donation::Table, Donation::id_donator)
                            .to(super::m20250323_041446_create_table_donator::Donator::Table, super::m20250323_041446_create_table_donator::Donator::Id)
                            .on_delete(ForeignKeyAction::NoAction)
                            .on_update(ForeignKeyAction::Cascade)   
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Donation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Donation {
    Table,
    Id,
    date,
    id_beneficiary,
    id_donator,
}
