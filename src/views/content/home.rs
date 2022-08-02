use rocket::get;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/")]
pub async fn index() -> Template {
    let mut context = HashMap::new();
 
    context.insert("title", "blog");
  
    let template = Template::render("index", context);
    template
}