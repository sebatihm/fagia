pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250323_041446_create_table_donator;
mod m20250323_044314_create_table_credentials;
mod m20250323_045440_create_table_beneficiary;
mod m20250324_041504_create_table_aliments;
mod m20250324_042935_create_table_donation;
mod m20250324_044202_create_table_aliment_per_donation;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250323_044314_create_table_credentials::Migration),
            Box::new(m20250323_041446_create_table_donator::Migration),
            Box::new(m20250323_045440_create_table_beneficiary::Migration),
            Box::new(m20250324_041504_create_table_aliments::Migration),
            Box::new(m20250324_042935_create_table_donation::Migration),
            Box::new(m20250324_044202_create_table_aliment_per_donation::Migration),
        ]
    }
}
