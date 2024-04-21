use sea_orm_migration::prelude::*;

use super::m20230905_000001_create_ingredient::Ingredient;
use super::m20230905_000002_create_recipe::Recipe;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecipeIngredient::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RecipeIngredient::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RecipeIngredient::IngredientId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RecipeIngredient::RecipeId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(RecipeIngredient::Unit).string().not_null())
                    .col(
                        ColumnDef::new(RecipeIngredient::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe_ingredient-ingredient_id")
                            .from(RecipeIngredient::Table, RecipeIngredient::IngredientId)
                            .to(Ingredient::Table, Ingredient::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe_ingredient-recipe_id")
                            .from(RecipeIngredient::Table, RecipeIngredient::RecipeId)
                            .to(Recipe::Table, Recipe::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RecipeIngredient::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RecipeIngredient {
    Table,
    Id,
    IngredientId,
    RecipeId,
    Unit,
    Quantity,
}
