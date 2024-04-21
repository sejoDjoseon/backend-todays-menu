use crate::entities::{prelude::*, *};
use crate::errors::ErrorResponder;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

#[get("/recipes")]
pub async fn get_recipes(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<recipe::Model>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let recipes: Vec<recipe::Model> = Recipe::find().all(db).await.map_err(ErrorResponder::from)?;

    Ok(Json(recipes))
}
