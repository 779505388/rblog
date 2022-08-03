use crate::service::views::link_serv::LinkPageData;
use rocket::get;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
#[get("/link")]
pub async fn index() -> Template {
    let render_data = LinkPageData::service_link().await;
    let mut context = HashMap::new();
    context.insert("render_data", render_data);
    println!("{:#?}", &context);
    let template = Template::render("content/link", &context);
    template
}
