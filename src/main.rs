use rocket::routes;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

mod views;
use views::content::home;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
    .mount("/", routes![home::index])
    .attach(Template::fairing())
    .launch().await?;
    Ok(())
}

