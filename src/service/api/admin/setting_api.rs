use crate::models::user::User;
use crate::CONFIG;
use crate::utils::{auth::UserAuth, csrf::CsrfStatus};
use md5;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::{ get, post, put, FromForm};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::fs;

//获取设置信息
#[get("/setting")]
pub async fn api_setting_get(
    cookies: &CookieJar<'_>,
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
) -> Value {
    let config = CONFIG.try_lock().unwrap();
    let user_id = cookies
        .get_private("login_user_id")
        .unwrap()
        .value()
        .to_string()
        .parse::<usize>()
        .unwrap();
    let user = User::get_user_by_user_id(&user_id).await;
    json!({"data":{
        "avatar_proxyz":config.avatar_proxyz,
        "article_num":config.article_num,
        "blog_name":config.blog_name,
        "user_name":user.username,
        "nick_name":config.nick_name,
        "domain":config.domain,
        "register":config.register,
        "github":config.github,
        "zhihu":config.github,
        "email":user.mail
    }})
}

#[derive(FromForm, Serialize, Deserialize, Clone, Debug)]
pub struct InfoForm {
    pub avatar_proxyz: Option<String>,
    pub article_num: Option<usize>,
    pub nick_name: Option<String>,
    pub blog_name: Option<String>,
    pub domain: Option<String>,
    pub register: Option<bool>,
    pub github: Option<String>,
    pub zhihu: Option<String>,
    pub email: Option<String>,
}
#[post("/setting/info", data = "<form_data>")]
pub async fn api_setting_post_info(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<InfoForm>,
) -> Value {
    let data = form_data.into_inner();
    let data_copy = data.clone();
    let email_hash = format!(
        "{:x}",
        md5::compute(&data_copy.email.clone().unwrap_or("".to_string()))
    );
    let text = format!(
        r#"[setting]
avatar_proxyz="{}"
article_num="{}"
nick_name="{}"
blog_name="{}"
domain ="{}"
email_hash = "{}" #自动填写
register="{}"
github="{}"
zhihu="{}"
email="{}"
    "#,
        data_copy
            .avatar_proxyz
            .unwrap_or("https://secure.gravatar.com/avatar/".to_string()),
        data_copy.article_num.unwrap_or(5),
        data_copy.nick_name.unwrap_or("用户未命名".to_string()),
        data_copy.blog_name.unwrap_or("博客未命名".to_string()),
        data_copy
            .domain
            .unwrap_or("https:://www.xxxxx.com".to_string()),
        email_hash.clone(),
        data_copy.register.unwrap_or(false),
        data_copy.github.unwrap_or("".to_string()),
        data_copy.zhihu.unwrap_or("".to_string()),
        data_copy.email.unwrap_or("".to_string())
    );
    fs::write("Blog.toml", text).unwrap();
    let setting = CONFIG.try_lock().unwrap();
    let mut blog_name = setting.blog_name.try_lock().unwrap();
    let mut avatar_proxyz = setting.avatar_proxyz.try_lock().unwrap();
    let mut article_num = setting.article_num.try_lock().unwrap();
    let mut nick_name = setting.nick_name.try_lock().unwrap();
    let mut domain = setting.domain.try_lock().unwrap();
    let mut email_hash1 = setting.email_hash.try_lock().unwrap();
    let mut register = setting.register.try_lock().unwrap();
    let mut github = setting.github.try_lock().unwrap();
    let mut zhihu = setting.zhihu.try_lock().unwrap();
    let mut email = setting.email.try_lock().unwrap();
    *blog_name = data.blog_name.unwrap_or("博客未命名".to_string());
    *avatar_proxyz = data
        .avatar_proxyz
        .unwrap_or("https://secure.gravatar.com/avatar/".to_string());
    *article_num = data.article_num.unwrap_or(5);
    *nick_name = data.nick_name.clone().unwrap_or("用户未命名".to_string());
    *domain = data.domain.unwrap_or("https:://www.xxxxx.com".to_string());
    *email_hash1 = email_hash.clone();
    *register = data.register.unwrap_or(false);
    *github = data.github.unwrap_or("".to_string());
    *zhihu = data.zhihu.unwrap_or("".to_string());
    *email = data.email.clone().unwrap_or("".to_string());
    // let user_id = cookies
    //     .get_private("login_user_id")
    //     .unwrap()
    //     .value()
    //     .to_string().parse::<usize>().unwrap();

    // let user = User {
    //     id: Some(user_id),
    //     username: None,
    //     password: None,
    //     nickname: Some(data.nick_name.clone().unwrap_or("用户未命名".to_string())),
    //     mail: Some(data.email.unwrap_or("".to_string())),
    //     mail_hash: Some(email_hash),
    // };
    // let status = User::update_user(user).await;
    json!({"status":"success","message":"修改用户信息成功！"})
}

#[derive(FromForm, Serialize, Deserialize, Clone, Debug)]
pub struct UserForm {
    pub nick_name: Option<String>,
    pub user_name: Option<String>,
    pub old_password: Option<String>,
    pub new_password: Option<String>,
    pub email: Option<String>,
}
#[put("/setting/user", data = "<form_data>")]
pub async fn api_setting_post_user(
    cookies: &CookieJar<'_>,
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<UserForm>,
) -> Value {
    let data = form_data.into_inner();
    let status = match data.new_password {
        Some(_) => true,
        None => false,
    };
    let user_id = cookies
        .get_private("login_user_id")
        .unwrap()
        .value()
        .to_string()
        .parse::<usize>()
        .unwrap();
    if status {
        let old_password = data.old_password.unwrap();
        let new_password = data.new_password.unwrap();
        let verify = User::verify_password(&user_id, &old_password).await;
        if verify {
            let encode_pwd = User::bcrypt(new_password.clone(), "salt".to_string());
            let user = User {
                id: Some(user_id),
                username: Some(data.user_name.unwrap()),
                password: Some(encode_pwd),
                nickname: Some(data.nick_name.unwrap_or("".to_string())),
                mail: Some(data.email.clone().unwrap_or("".to_string())),
                mail_hash: Some(format!(
                    "{:x}",
                    md5::compute(&data.email.clone().unwrap_or("".to_string()))
                )),
            };
            let status = User::update_user(user).await;
            if status {
                json!({"status":"success","message":"修改用户信息及密码成功！"})
            } else {
                json!({"status":"error","message":"修改用户信息及密码错误！"})
            }
        } else {
            json!({"status":"error","message":"旧密码错误！"})
        }
    } else {
        let user = User {
            id: Some(user_id),
            username: Some(data.user_name.unwrap()),
            password: None,
            nickname: Some(data.nick_name.unwrap_or("".to_string())),
            mail: Some(data.email.clone().unwrap_or("".to_string())),
            mail_hash: Some(format!(
                "{:x}",
                md5::compute(&data.email.clone().unwrap_or("".to_string()))
            )),
        };
        let status = User::update_user(user).await;
        if status {
            json!({"status":"success","message":"修改用户信息成功！"})
        } else {
            json!({"status":"error","message":"修改用户信息错误！"})
        }
    }
}
