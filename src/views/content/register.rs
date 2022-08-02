use crate::models::user::User;
use crate::CONFIG;
use rocket::figment::Figment;
use rocket::figment::providers::{Toml, Format};
use rocket::form::Form;
use rocket::route;
use rocket::serde::json::Json;
use rocket::serde::json::{serde_json::json, Value};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
#[route(GET, uri = "/register")]
pub async fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("users", "users");
    println!("{:#?}", &context);

    Template::render("content/register", context)
}

#[route(POST, uri = "/register", data = "<form_data>")]
pub async fn post(form_data: Json<User>) -> Value {
    let user_data = form_data.into_inner();
    let data = Figment::from(Toml::file("Blog.toml").nested()).select("setting");
    let register = data
    .find_value("register")
    .ok()
    .unwrap()
    .into_string()
    .unwrap()
    .parse::<bool>()
    .unwrap_or(false);
    if register {
        let ststus = User::register_user(user_data).await;
        if ststus{
            return json!({"status":"success","message":"注册成功！"});
        }else{
            return json!({"status":"error","message":"用户名或邮箱已经存在！"});
        }
    }else{
        json!({"ststus":"error","message":"注册已经关闭！"})
    }    
    
}
