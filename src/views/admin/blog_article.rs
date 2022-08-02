use crate::{
    service::views::admin::blog_article_serv::BlogArticle,
    utils::{
        auth::UserAuth,
        csrf::CsrfStatus,
    }, models::category::Category,
};
use rocket::http::CookieJar;
use rocket::{get, post};
use rocket_dyn_templates::Template;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

//文章列表
#[get("/article-list")]
pub async fn list( _user_auth: UserAuth, cookies: &CookieJar<'_>) -> Template {
    let mut context = HashMap::new();
    let key = match cookies.get_private("csrf_key") {
        Some(i) => i.value().to_string(),
        None => "None Key".to_string(),
    };
    let csrf_token = CsrfStatus::encrypt_csrf(key).await;
    context.insert("csrf_token", csrf_token);
    let template = Template::render("admin/blog-list", &context);
    template
}
#[post("/article-list")]
pub async fn post_article(_user_auth: UserAuth, csrf_token: CsrfStatus) -> Value {
    let list = BlogArticle::article_list().await;
    json!({ "data": list })
}

#[get("/article-modify/<id>")]
pub async fn modify(id:usize, _user_auth: UserAuth, cookies: &CookieJar<'_>) -> Template {
    let mut context = HashMap::new();
    let key = match cookies.get_private("csrf_key") {
        Some(i) => i.value().to_string(),
        None => "None Key".to_string(),
    };
    let csrf_token = CsrfStatus::encrypt_csrf(key).await;
    context.insert("csrf_token", csrf_token);
    let template = Template::render("admin/blog-modify", &context);
    template
}

#[post("/article-modify/<id>")]
pub async fn post_modify(id:usize,_user_auth: UserAuth, csrf_token: CsrfStatus) -> Value {
    let article = BlogArticle::article_modify(&id).await;
    let category = Category::get_all().await;
    json!({
        "data":article,
        "category":category
    })
}

#[get("/article-add")]
pub async fn add(_user_auth: UserAuth, cookies: &CookieJar<'_>) -> Template {
    let mut context = HashMap::new();
    let key = match cookies.get_private("csrf_key") {
        Some(i) => i.value().to_string(),
        None => "None Key".to_string(),
    };
    let csrf_token = CsrfStatus::encrypt_csrf(key).await;
    context.insert("csrf_token", csrf_token);
    let template = Template::render("admin/blog-write", &context);
    template
}