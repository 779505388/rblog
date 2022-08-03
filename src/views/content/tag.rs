use crate::service::views::tag_serv::{TagListData, TagPageData};
use crate::utils::response::HandleResponse;
use rocket::{get, http::Status};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/tag")]
pub async fn index_list() -> Template {
    let render_data = TagListData::get_all().await;
    let mut context = HashMap::new();
    context.insert("render_data", render_data);
    println!("{:#?}", &context);
    Template::render("content/tags", context)
}

#[get("/tag/<tag>?<page>")]
pub async fn index_tag(tag: Option<&str>, page: Option<usize>) -> HandleResponse {
    let page_no = match page {
        Some(i) => i,
        None => 1,
    };
    let is_none = match tag {
        Some(_) => false,
        None => true,
    };
    if is_none {
        return HandleResponse::Status(Status::NotFound);
    };
    let render_data = TagPageData::service_tag(&tag.unwrap(),page_no).await;
    let mut context = HashMap::new();
    context.insert("render_data", render_data);
    println!("{:#?}", &context);
    HandleResponse::Template(Template::render("content/tag", context))
}
