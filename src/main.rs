mod entities;
mod errors;
mod routes;
mod setup;

use rocket::*;

#[get("/")]
async fn index() -> &'static str {
    "Hello, todays_menu!"
}

#[launch]
async fn rocket() -> _ {
    let db = match setup::set_up_db().await {
        Ok(db) => db,
        Err(err) => panic!("{}", err),
    };

    rocket::build().manage(db).mount(
        "/",
        routes![
            index,
            routes::recipe::get_recipes,
            routes::ingrdient::get_ingredients
        ],
    )
}
