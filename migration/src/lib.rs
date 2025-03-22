pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20250322_001132_create_table_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250322_001132_create_table_users::Migration),
        ]
    }
}
