use crate::models::article::Article;
use crate::models::category::CategoryArticle;
use crate::models::tag::{Tag, TagArticle};
use crate::{
    models::category::Category,
    service::views::admin::blog_article_serv::BlogArticle,
    utils::{auth::UserAuth, csrf::CsrfStatus},
};
use rbatis::{DateTimeNative, DateTimeUtc};
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostArticle {
    pub id: Option<usize>,
    pub title: Option<String>,
    pub brief: Option<String>,
    pub user_id: Option<usize>,
    pub url_en: Option<String>,
    pub text: Option<String>,
    pub template: Option<String>,
    pub image_url: Option<String>,
    pub like_count: Option<usize>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub views: Option<usize>,
    pub created: Option<DateTimeUtc>,
    pub modified: Option<DateTimeUtc>,
}

//提交文章
#[post("/article", data = "<form_data>")]
pub async fn api_article_post(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<PostArticle>,
) -> Value {
    let data = form_data.into_inner();
    let data_copy = data.clone();
    let article_data = Article {
        title: data.title,
        brief: data.brief,
        text: data.text,
        template: data.template,
        image_url: data.image_url,
        category: data.category,
        url_en: data.url_en,
        created: Some(DateTimeNative::now()),
        modified: Some(DateTimeNative::now()),
        id: None,
        user_id: None,
        like_count: None,
        views: None,
    };
    let r = Article::commit_article(article_data).await;
    let article_status = match r {
        Ok(_) => true,
        Err(_) => false,
    };
    if !article_status {
        return json!({"status":"error","message":"文章提交发生错误！"});
    };
    let article_id = Article::get_article(&data_copy.url_en.unwrap())
        .await
        .unwrap()
        .unwrap()
        .id;
    let category_result = CategoryArticle::commit_category_name_and_article_id(
        &data_copy.category.unwrap(),
        article_id.clone(),
    )
    .await;
    let category_status = match category_result {
        Ok(_) => true,
        Err(_) => false,
    };
    if !category_status {
        return json!({"status":"error","message":"文章分类提交发生错误！"});
    };
    let commit_tag_status = TagArticle::commit_tag_article(article_id, data_copy.tags).await;
    if !commit_tag_status {
        return json!({"status":"error","message":"文章标签提交发生错误！"});
    };
    return json!({"status":"success","message":"文章提交成功！"});
}

#[delete("/article/<id>")]
pub async fn api_article_delete(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    id: usize,
) -> Value {
    let result = Article::get_article_by_id(&id).await;
    let is_none = match result {
        Ok(_) => false,
        Err(_) => true,
    };
    if is_none {
        return json!({"status":"error","message":"删除的文章不存在！"});
    };
    let article_id = result.unwrap().unwrap().id.unwrap();
    let tag_del_status = TagArticle::delete_tag_article_by_article_id(&article_id).await;
    if !tag_del_status {
        return json!({"status":"error","message":"删除标签失败！"});
    };
    let category_del_status =
        CategoryArticle::delete_category_article_by_article_id(&article_id).await;
    if !category_del_status {
        return json!({"status":"error","message":"删除分类失败！"});
    };
    let article_del_status = Article::delete_article_by_id(&article_id).await;
    if !article_del_status {
        return json!({"status":"error","message":"删除文章失败失败！"});
    };
    let list = BlogArticle::article_list().await;
    return json!({"status":"success","message":"删除文章成功！","data":list});
}

//修改文章
#[put("/article/<id>", data = "<form_data>")]
pub async fn api_article_put(
    _user_auth: UserAuth,
    _csrf_status: CsrfStatus,
    form_data: Json<PostArticle>,
    id: usize,
) -> Value {
    println!("{:#?}", &form_data);
    let data = form_data.into_inner();
    let article_data = Article {
        id: Some(id.clone()),
        brief: data.brief,
        user_id: None,
        url_en: data.url_en,
        text: data.text,
        template: data.template,
        like_count: None,
        category: data.category.clone(),
        views: None,
        created: None,
        modified: Some(DateTimeNative::now()),
        title: data.title,
        image_url: data.image_url,
    };
    let article_status = Article::update_article(&article_data).await;
    if !article_status {
      return  json!({"status":"error","message":"修改文章错误"});
    };
    let category_status = CategoryArticle::update_category(id.clone(), data.category).await;
    if !category_status {
        return  json!({"status":"error","message":"修改文章分类失败"});
      };
    let tag_status = TagArticle::update_tags(id, data.tags).await;
    if !tag_status {
        return  json!({"status":"error","message":"修改文章标签失败"});
      };  
    json!({"status":"success","message":"修改文章内容成功！"})
}

//获取文章信息文章
#[get("/article/<id>")]
pub async fn api_article_get(_user_auth: UserAuth, _csrf_status: CsrfStatus, id: usize) -> Value {
    let article = BlogArticle::article_modify(&id).await;
    let category = Category::get_all().await;
    let tag_id = TagArticle::get_tag_list_by_article_id(&id).await;
    let mut tags = vec![];
    let tag_status = match tag_id {
        Ok(_) => true,
        Err(_) => false,
    };
    if tag_status {
        for item in tag_id.unwrap() {
            let tag = Tag::get_list_by_id(&item.tag_id.unwrap()).await;
            tags.push(tag)
        }
    };
    json!({
        "data":article,
        "tags":tags,
        "category":category
    })
}
