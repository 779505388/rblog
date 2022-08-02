use crate::models::user::User;
use crate::utils::response::HandleResponse;
use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::route;
use rocket::serde::json::Json;
use rocket::serde::json::{serde_json::json, Value};
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[route(GET, uri = "/login")]
pub async fn index(cookies: &CookieJar<'_>) -> HandleResponse {
    let mut context = HashMap::new();
    let login = cookies.get_private("login_user_id");
    let login_status = match login {
        Some(_) => true,
        None => false,
    };
    context.insert("users", "users");
    println!("{:#?}", &login);
    if login_status {
        return HandleResponse::Redirect(Redirect::to("/admin/dashboard"));
    } else {
        return HandleResponse::Template(Template::render("content/login", context));
    };
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoginUser {
    mail: String,
    password: String,
    remember: bool,
}

#[route(POST, uri = "/login", data = "<form_data>")]
pub async fn post(cookies: &CookieJar<'_>, form_data: Json<LoginUser>) -> Value {
    let user_data = form_data.into_inner();
    let info = User::login_blog(&user_data.mail, &user_data.password).await;
    let login = info.get("status").clone().unwrap().as_str().unwrap();
    if login == "success" {
        let user = User::get_user_by_email(&user_data.mail).await;
        let mut ck = Cookie::new("login_user_id", user.id.unwrap().to_string());
        if !user_data.remember {
            ck.set_expires(None);
        };
        cookies.add_private(ck);
    };
    return info;
}

#[route(GET, uri = "/login/out")]
pub async fn login_out(cookies: &CookieJar<'_>) -> HandleResponse {
    cookies.remove_private(Cookie::named("login_user_id"));
     return HandleResponse::Redirect(Redirect::to("/login"));

}