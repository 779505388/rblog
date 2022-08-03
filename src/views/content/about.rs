use rocket::{get, http::Status};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/about")]
pub async fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("render_data", "render_data");
    println!("{:#?}", &context);
    Template::render("content/about", context)
}