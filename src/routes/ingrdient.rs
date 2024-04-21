use crate::entities::{prelude::*, *};
use crate::errors::ErrorResponder;
use rocket::serde::json::Json;
use rocket::*;
use sea_orm::*;

#[get("/ingredients")]
pub async fn get_ingredients(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<ingredient::Model>>, ErrorResponder> {
    let db = db as &DatabaseConnection;

    let ingredients: Vec<ingredient::Model> = Ingredient::find()
        .all(db)
        .await
        .map_err(ErrorResponder::from)?;

    Ok(Json(ingredients))
}
