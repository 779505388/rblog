/*
url：/tag 对应的服务；
1. /tag/list 显示所有标签
2./tag?<tag> 显示标签对应文章
*/

use crate::models::tag::TagArticle;
use crate::{
    models::{article::Article, category::Category, tag::Tag},
    RB,
};
use rbatis::crud::CRUD;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagListData {
    pub tags: Vec<Tag>, // 标签列表
}

impl TagListData {
    pub async fn get_all() -> Vec<Tag> {
        RB.fetch_list::<Tag>().await.unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TagPageData {
    pub articles: Vec<Article>,   // 文章列表
    pub categorys: Option<Vec<Category>>, //标签列表
    pub page_total: usize,          //总共页码数
    pub page_size: usize,           //页面显示文章数量
    pub page: usize,                //当前页面数
}

impl TagPageData {
    pub async fn service_tag(name: &str,page:usize) -> TagPageData {
        let tag = Tag::get_by_name(name).await;
        let article_id = TagArticle::get_list_by_tag_id(&tag.id.unwrap()).await;
        let mut id_list = Vec::new();
        for i in article_id {
            id_list.push(i.article_id.unwrap())
        };
        let categorys = Category::get_all().await;
        let page_total = if id_list.len() % 5 == 0 {
            id_list.len() / 5
        } else {
            id_list.len() / 5 + 1
        };
        let articles = Article::get_article_list_by_id(page, 5, id_list).await;
        TagPageData {
            articles: articles,
            categorys: categorys,
            page_total: page_total as usize,
            page_size: 5,
            page: page,
        }
    }
}
