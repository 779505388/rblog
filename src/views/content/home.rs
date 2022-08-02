use rocket::get;
use std::collections::HashMap;
use rocket_dyn_templates::Template;
use crate::service::views::home_serv::HomePageData;



#[get("/?<page>")]
pub async fn index(page:Option<usize>) -> Template {
    let page_num = match page {
        Some(i) => i,
        None =>1
    };
    let page_data = HomePageData::service_home(page_num).await;
    let mut context =HashMap::new();
    context.insert("render_data",page_data);
    let template = Template::render("content/index", context);
    template
}