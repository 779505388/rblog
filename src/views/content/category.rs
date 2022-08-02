use crate::{service::views::category_serv::CategoryPageData};
use rocket::get;
use rocket_dyn_templates::Template;
use std::collections::HashMap;



#[get("/category/<url>?<page>")]
pub async fn index(url: &str,page:Option<usize>) -> Template {
    let page_no = match page {
        Some(i)=>i,
        None=>1
    };
    let render_data = CategoryPageData::service_category(url,page_no).await;
    let mut context = HashMap::new();
    context.insert("render_data", render_data);
    Template::render("content/category", context)
}