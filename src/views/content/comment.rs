use crate::models::comment::Comment;
use crate::service::views::comment_serv::CommentData;
use rocket::http::CookieJar;
use rocket::post;
use rocket::serde::json::{json, Json, Value};

#[post("/comment?<captcha>", format = "json", data = "<comment_data>")]
pub async fn index(comment_data: Json<Comment>, cookies: &CookieJar<'_>, captcha: &str) -> Value {
    let captcha_status = cookies.get_private("content/captcha").unwrap();
    println!("{:#?}---{:#?}",&captcha_status.value(),captcha);
    if captcha.to_ascii_uppercase() == captcha_status.value().to_ascii_uppercase() {
        let login_status = cookies.get_private("login_user_id");
        let is_admin = match login_status {
            Some(_) => true,
            None => false,
        };
        let info = CommentData::service_comment(comment_data.into_inner(), is_admin).await;
        return json!(info);
    };
    return json!({
        "status":"error",
        "message":"验证码错误",
    });
}
