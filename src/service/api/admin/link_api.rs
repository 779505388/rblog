use crate::models::link::Link;
use crate::{
    models::{category::Category, comment::Comment},
    service::views::admin::blog_article_serv::BlogArticle,
    utils::{auth::UserAuth, csrf::CsrfStatus},
};
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::{delete, get, post, FromForm, put};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use rbatis::DateTimeNative;


//获取所有友链
#[get("/link/all")]
pub async fn api_link_get_all(_user_auth: UserAuth, _csrf_status: CsrfStatus) -> Value {
    let links = Link::get_all().await;
    let status = match links {
        Ok(_) => true,
        Err(_) => false,
    };
    if status {
        println!("{:#?}", &links);
        json!({ "status": "success","data": links.unwrap() })
    } else {
        json!({ "status": "error" })
    }
}

#[derive(FromForm, Serialize, Deserialize, Clone, Debug)]
pub struct LinkForm {
    pub id: Option<usize>,
    pub name: Option<String>,
    pub link: Option<String>,
    pub avatar: Option<String>,
    pub brief: Option<String>,
}

//提交新友链
#[post("/link", data = "<form_data>")]
pub async fn api_link_post(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<LinkForm>,
) -> Value {
    let data = form_data.into_inner();
    let link_data = Link {
        id: None,
        name: data.name,
        link: data.link,
        avatar: data.avatar,
        brief:data.brief,
        created: Some(DateTimeNative::now()),
    };
    let commit_status = Link::commit_link(link_data).await;
    if commit_status {
        json!({ "status": "success","message": "提交友链成功！" })
    } else {
        json!({ "status": "error","message": "提交友链失败！" })
    }
}

//更新友链
#[put("/link", data = "<form_data>")]
pub async fn api_link_put(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<LinkForm>,
) -> Value {
    let data = form_data.into_inner();
    let link_data = Link {
        id: data.id,
        name: data.name,
        link: data.link,
        avatar: data.avatar,
        brief:data.brief,
        created: None,
    };
    let update_status = Link::update_link(link_data).await;
    if update_status {
        json!({ "status": "success","message": "更新友链成功！" })
    } else {
        json!({ "status": "error","message": "更新友链失败！" })
    }
}

//删除友链
#[delete("/link", data = "<form_data>")]
pub async fn api_link_delete(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<LinkForm>,
) -> Value {
    let data = form_data.into_inner();
    let link_data = Link {
        id: data.id,
        name: data.name,
        link: data.link,
        avatar: data.avatar,
        brief:data.brief,
        created: None,
    };
    let update_status = Link::delete_link(link_data).await;
    if update_status {
        json!({ "status": "success","message": "删除友链成功！" })
    } else {
        json!({ "status": "error","message": "删除友链失败！" })
    }
}