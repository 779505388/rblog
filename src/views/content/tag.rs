use crate::models::category::Category;
use crate::models::tag::Tag;
use crate::service::views::tag_serv::{TagListData, TagPageData};
use crate::utils::response::HandleResponse;
use rocket::{get, http::Status};
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TagData{
    pub tags:Option<Vec<Tag>>,
    pub categorys:Option<Vec<Category>>
}

#[get("/tag")]
pub async fn index_list() -> Template {
    let categorys=Category::get_all().await;
    let tags = TagListData::get_all().await;
    let render_data= TagData{
        tags:Some(tags),
        categorys,
    };
    let mut context = HashMap::new();
    context.insert("render_data", render_data);
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
    HandleResponse::Template(Template::render("content/tag", context))
}
