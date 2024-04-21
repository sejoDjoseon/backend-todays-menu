pub use sea_orm_migration::prelude::*;

mod m20230905_000001_create_ingredient;
mod m20230905_000002_create_recipe;
mod m20230905_000003_create_recipe_ingredient;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230905_000001_create_ingredient::Migration),
            Box::new(m20230905_000002_create_recipe::Migration),
            Box::new(m20230905_000003_create_recipe_ingredient::Migration),
        ]
    }
}
