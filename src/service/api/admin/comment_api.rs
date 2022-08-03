use crate::{
    models::comment::Comment,
    utils::{auth::UserAuth, csrf::CsrfStatus},
};
use rocket::serde::json::Json;
use rocket::{delete, get, FromForm};
use serde::{Serialize, Deserialize};
use serde_json::json;
use serde_json::Value;

//获取所有评论
#[get("/comment/all")]
pub async fn api_comment_get_all(_user_auth: UserAuth, _csrf_status: CsrfStatus) -> Value {
    let comments = Comment::get_comment_all().await;
    let status = match comments {
        Ok(_) => true,
        Err(_) => false,
    };
    if status {
        println!("{:#?}", &comments);
        json!({ "status": "success","data": comments.unwrap() })
    } else {
        json!({ "status": "error" })
    }
}


#[derive(FromForm,Serialize, Deserialize, Clone, Debug)]
pub struct DeleteList {
    delete_list: Option<Vec<usize>>,
}

//获取所有分类
#[delete("/comment", data = "<form_data>")]
pub async fn api_comment_delete(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<DeleteList>,
) -> Value {
    let data = form_data.into_inner().delete_list;
    let status = match data {
        Some(_) => true,
        None => false,
    };
    if status {
        println!("{:#?}", &data);
        let comment_status = Comment::delete_comment_list(&data.unwrap()).await;
        if comment_status{
            return json!({ "status": "success","message":"评论删除成功！"});
        }else{};
        return json!({ "status": "error","message":"评论删除失败！"});
    } else {
        json!({ "status": "error","message":"删除内容为空！" })
    }
}
