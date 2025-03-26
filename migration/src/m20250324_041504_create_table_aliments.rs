use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
  

        manager
            .create_table(
                Table::create()
                    .table(Aliments::Table)
                    .if_not_exists()
                    .col(pk_auto(Aliments::Id).not_null())
                    .col(string(Aliments::name).not_null())
                    .col(string(Aliments::r#type).not_null())
                    .col(text(Aliments::description).not_null())
                    .col(integer(Aliments::lots).not_null())
                    .col(date(Aliments::caducity_date).not_null())
                    .col(integer(Aliments::id_donator).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-aliments-benefactor")
                            .from(Aliments::Table, Aliments::id_donator)
                            .to(super::m20250323_045440_create_table_beneficiary::Beneficiary::Table, super::m20250323_045440_create_table_beneficiary::Beneficiary::Id)
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
            .drop_table(Table::drop().table(Aliments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Aliments {
    Table,
    Id,
    name,
    r#type,
    description,
    lots,
    caducity_date,
    id_donator

}
