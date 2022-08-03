use crate::{
    models::tag::{Tag, TagArticle},
    utils::{auth::UserAuth, csrf::CsrfStatus},
};
use rocket::serde::json::Json;
use rocket::{delete, get, FromForm, put};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;



//获取所有标签
#[get("/tag/all")]
pub async fn api_tag_get_all(_user_auth: UserAuth, _csrf_status: CsrfStatus) -> Value {
    let tags = Tag::get_all().await;
    let status = match tags {
        Ok(_) => true,
        Err(_) => false,
    };
    if status {
        println!("{:#?}", &tags);
        json!({ "status": "success","data": tags.unwrap() })
    } else {
        json!({ "status": "error" })
    }
}

#[derive(FromForm, Serialize, Deserialize, Clone, Debug)]
pub struct TagForm {
    pub id: Option<usize>,
    pub name: Option<String>,

}

//更新标签
#[put("/tag", data = "<form_data>")]
pub async fn api_tag_put(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<TagForm>,
) -> Value {
    println!("{:#?}",&form_data);
    let data = form_data.into_inner();
    let tag_data = Tag {
        id: data.id,
        name: data.name,
    };
    let update_status = Tag::update_tag(tag_data).await;
    if update_status {
        json!({ "status": "success","message": "更新标签成功！" })
    } else {
        json!({ "status": "error","message": "更新标签失败！" })
    }
}

//删除标签
#[delete("/tag", data = "<form_data>")]
pub async fn api_tag_delete(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<TagForm>,
) -> Value {
    println!("{:#?}",&form_data);
    let data = form_data.into_inner();
    let tag_data = Tag {
        id: data.id,
        name: data.name,
    };
    let delete_status = Tag::delete_tag(&tag_data.id.unwrap()).await;
    if !delete_status {
       return json!({ "status": "error","message": "删除标签失败！" });
    } ;
    let delete_status = TagArticle::delete_tag_article_by_tag_id(&tag_data.id.unwrap()).await;
    if !delete_status {
       return json!({ "status": "error","message": "删除标签&文章表内容失败！" });
    };
    json!({ "status": "success","message": "删除标签成功！" })
}