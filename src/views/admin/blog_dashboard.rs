use crate::{
    service::views::admin::dashboard_serv::DashboardInfo,
    utils::{
        auth::UserAuth,
        csrf::CsrfStatus,
    },
};
use rocket::http::CookieJar;
use rocket::{get, post};
use rocket_dyn_templates::Template;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
#[get("/dashboard")]
pub async fn index( _user_auth: UserAuth, cookies: &CookieJar<'_>) -> Template {
    let mut context = HashMap::new();
    let key = match cookies.get_private("csrf_key") {
        Some(i) => i.value().to_string(),
        None => "None Key".to_string(),
    };
    let csrf_token = CsrfStatus::encrypt_csrf(key).await;
    context.insert("csrf_token", csrf_token);
    let template = Template::render("admin/blog-dashboard", &context);
    template
}
#[post("/dashboard/info")]
pub async fn info(_user_auth: UserAuth, _csrf_token: CsrfStatus) -> Value {
    let info = DashboardInfo::service_dashboard().await;
    json!({ "data": info })
}
