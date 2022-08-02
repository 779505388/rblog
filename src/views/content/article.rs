use crate::{service::views::article_serv::ArticlePageData, utils::response::HandleResponse, AVATAR};
use rocket::{get, http::Status};
use rocket_dyn_templates::Template;
use std::collections::HashMap;

#[get("/article/<url>")]
pub async fn index(url: &str) -> HandleResponse {
    let render_data = ArticlePageData::service_article(url).await;
    let is_null = match &render_data.article {
        Some(_) => false,
        None => true,
    };
    if is_null {
        return HandleResponse::Status(Status::NotFound);
    };
    println!("{:#?}",AVATAR);
    let mut context = HashMap::new();
    context.insert("render_data", render_data);
    HandleResponse::Template(Template::render("content/article", context))
}
