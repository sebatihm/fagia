pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250323_041446_create_table_donator;
mod m20250323_044314_create_table_credentials;
mod m20250323_045440_create_table_benefactor;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20250323_041446_create_table_donator::Migration),
            Box::new(m20250323_044314_create_table_credentials::Migration),
            Box::new(m20250323_045440_create_table_benefactor::Migration),
        ]
    }
}
