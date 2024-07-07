mod m20240101_000000_database_setup;
mod m20240101_000001_create_example;

use sea_orm_migration::prelude::*;

pub struct DbMigrator;

#[async_trait::async_trait]
impl MigratorTrait for DbMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000000_database_setup::Migration),
            Box::new(m20240101_000001_create_example::Migration),
        ]
    }
}
