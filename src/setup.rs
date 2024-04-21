use sea_orm::*;

const DATABASE_URL: &str = "postgres://sejo:admin1234@localhost:5432";
const DB_NAME: &str = "todays_menu";

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    // let db = Database::connect(DATABASE_URL).await?;

    // db.execute(Statement::from_string(
    //     db.get_database_backend(),
    //     format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
    // ))
    // .await?;
    // db.execute(Statement::from_string(
    //     db.get_database_backend(),
    //     format!("CREATE DATABASE \"{}\";", DB_NAME),
    // ))
    // .await?;

    let url = format!("{}/{}", DATABASE_URL, DB_NAME);
    let db = Database::connect(&url).await?;

    Ok(db)
}

// mod entities;
// use entities::{prelude::*, *};

// let good_recipe = recipe::ActiveModel {
//     name: ActiveValue::Set("Good Recipe".to_owned()),
//     ..Default::default()
// };

// let res = Recipe::insert(good_recipe).exec(db).await?;

// let sad_recipe = recipe::ActiveModel {
//     id: ActiveValue::Set(res.last_insert_id),
//     name: ActiveValue::Set("Sad Recipe".to_owned()),
// };
// sad_recipe.update(db).await?;

// let good_ingredient = ingredient::ActiveModel {
//     name: ActiveValue::Set("Good Ingrdient".to_owned()),
//     ..Default::default()
// };
// let res_ingedient = Ingredient::insert(good_ingredient).exec(db).await?;

// let relation = recipe_ingredient::ActiveModel {
//     ingredient_id: ActiveValue::Set(res_ingedient.last_insert_id),
//     recipe_id: ActiveValue::Set(res.last_insert_id),
//     unit: ActiveValue::Set("pellizcos".to_owned()),
//     quantity: ActiveValue::Set(1),
//     ..Default::default()
// };

// RecipeIngredient::insert(relation).exec(db).await?;
