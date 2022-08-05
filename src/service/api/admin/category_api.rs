use crate::models::article::Article;
use crate::models::category::CategoryArticle;
use crate::{
    models::category::Category,
    utils::{auth::UserAuth, csrf::CsrfStatus},
};
use rocket::{delete, get, post, put};
use rocket::{serde::json::Json, FromForm};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

//获取所有分类
#[get("/category/all")]
pub async fn api_category_get_all(_user_auth: UserAuth, _csrf_status: CsrfStatus) -> Value {
    let category = Category::get_all().await;
    json!({ "data": category })
}

#[derive(FromForm, Serialize, Deserialize, Clone, Debug)]
pub struct CategoryData {
    id: Option<usize>,
    category: Option<String>,
}

//添加文章分类
#[post("/category", data = "<form_data>")]
pub async fn api_category_post(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<CategoryData>,
) -> Value {
    let data = form_data.into_inner().category;
    let status = match data {
        Some(_) => true,
        None => false,
    };
    if !status {
        return json!({"status":"error","message":"提交内容为空！"});
    };
    let category_status = Category::commit_category(data.unwrap()).await;
    if !category_status {
        return json!({"status":"error","message":"新增文章分类失败！"});
    };

    json!({"status":"success","message":"新增文章分类成功！"})
}

//删除文章分类
#[delete("/category", data = "<form_data>")]
pub async fn api_category_delete(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<CategoryData>,
) -> Value {
    let data = form_data.into_inner().category;
    let status = match data {
        Some(_) => true,
        None => false,
    };
    if !status {
        return json!({"status":"error","message":"提交内容为空！"});
    };
    let category = Category::get_by_name(&data.unwrap()).await;
    let status = match category {
        Ok(_) => true,
        Err(_) => false,
    };
    if !status {
        return json!({"status":"error","message":"删除内容不存在！"});
    };
    let category_id = category.unwrap().id.unwrap();
    let category_status = Category::delete_category_by_id(&category_id).await;
    if !category_status {
        return json!({"status":"error","message":"删除分类失败！"});
    };
    let category_status =
        CategoryArticle::delete_category_article_by_category_id(&category_id).await;
    if !category_status {
        return json!({"status":"error","message":"删除文章与分类关联表失败！"});
    };
    json!({"status":"success","message":"删除文章分类成功！"})
}

//修改文章分类
#[put("/category", data = "<form_data>")]
pub async fn api_category_put(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<CategoryData>,
) -> Value {
    let data = form_data.clone().into_inner().category;
    let category_id = form_data.into_inner().id;
    let status = match data {
        Some(_) => true,
        None => false,
    };
    if !status {
        return json!({"status":"error","message":"提交内容为空！"});
    };
    let category_status =
        Category::update_category(category_id.unwrap().clone(), data.as_ref().unwrap().clone()).await;
    if !category_status {
        return json!({"status":"error","message":"修改分类内容失败！"});
    };
    let id_list = CategoryArticle::get_article_id_list_by_category_id(&category_id.unwrap()).await;
    let status = match id_list {
        Some(_) => true,
        None => false,
    };
    if !status {
        return json!({"status":"error","message":"通过分类id无法找到文章id！"});
    };
    let id_list = id_list.unwrap();
    let status = Article::update_article_category(id_list, data.unwrap()).await;
    if !status {
        return json!({"status":"error","message":"修改文章-分类失败！"});
    };
    return json!({"status":"success","message":"修改文章分类成功！"});
}
