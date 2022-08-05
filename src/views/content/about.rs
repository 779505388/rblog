use rocket::get;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/about")]
pub async fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("render_data", "render_data");
    Template::render("content/about", context)
}
