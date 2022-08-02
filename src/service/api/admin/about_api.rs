use crate::models::link::Link;
use crate::{
    models::{category::Category, comment::Comment},
    service::views::admin::blog_article_serv::BlogArticle,
    utils::{auth::UserAuth, csrf::CsrfStatus},
};
use rbatis::DateTimeNative;
use rocket::data::FromData;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, FromForm};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

#[get("/about")]
pub async fn api_about_get(_user_auth: UserAuth, _csrf_status: CsrfStatus) -> Value {
    let meta = fs::read_to_string("template/content/include/meta.html.tera").unwrap();
    let script = fs::read_to_string("template/content/include/script.html.tera").unwrap();

    json!({ "status": "success","data": {"meta":meta,"script":script} })
}

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct AbotForm {
    meta: Option<String>,
    script: Option<String>,
}
#[post("/about", data = "<form_data>")]
pub async fn api_about_post(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<AbotForm>,
) -> Value {
    let data = form_data.into_inner();
    let meta = fs::write(
        "template/content/include/meta.html.tera",
        data.meta.unwrap_or("".to_string()),
    )
    .unwrap();
    let script = fs::write(
        "template/content/include/script.html.tera",
        data.script.unwrap_or("".to_string()),
    )
    .unwrap();

    json!({ "status": "success","message":"修改成功！"})
}
