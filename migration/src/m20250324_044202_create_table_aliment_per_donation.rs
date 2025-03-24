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
                    .table(AlimentPerDonation::Table)
                    .if_not_exists()
                    .col(integer(AlimentPerDonation::id_aliments).not_null())
                    .col(integer(AlimentPerDonation::id_donation).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-aliment-donation")
                            .from(AlimentPerDonation::Table, AlimentPerDonation::id_donation)
                            .to(super::m20250324_042935_create_table_donation::Donation::Table, super::m20250324_042935_create_table_donation::Donation::Id) 
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)   
                    )

                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-aliment-aliment")
                            .from(AlimentPerDonation::Table, AlimentPerDonation::id_aliments)
                            .to(super::m20250324_041504_create_table_aliments::Aliments::Table, super::m20250324_041504_create_table_aliments::Aliments::Id) 
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
            .drop_table(Table::drop().table(AlimentPerDonation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AlimentPerDonation {
    Table,
    id_aliments,
    id_donation,
}
