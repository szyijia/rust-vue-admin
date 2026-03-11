mod m20240101_000001_create_tables;
mod m20240101_000002_seed_data;

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_tables::Migration),
            Box::new(m20240101_000002_seed_data::Migration),
        ]
    }
}
