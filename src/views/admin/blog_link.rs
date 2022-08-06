use crate::utils::{auth::UserAuth, csrf::CsrfStatus};
use rocket::get;
use rocket::http::CookieJar;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
//文章列表
#[get("/link")]
pub async fn index(_user_auth: UserAuth, cookies: &CookieJar<'_>) -> Template {
    let mut context = HashMap::new();
    let key = match cookies.get_private("csrf_key") {
        Some(i) => i.value().to_string(),
        None => "None Key".to_string(),
    };
    let csrf_token = CsrfStatus::encrypt_csrf(key).await;
    context.insert("csrf_token", csrf_token);
    let template = Template::render("admin/blog-link", &context);
    template
}
