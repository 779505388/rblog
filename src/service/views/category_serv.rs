/*
url：/category 对应的服务；
*/

use crate::models::category::Category;
use crate::models::{article::Article, category::CategoryArticle};
use rbatis::{IPage, IPageRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CategoryPageData {
    pub articles: Option<Vec<Article>>,   // 文章列表
    pub categorys: Option<Vec<Category>>, //标签列表
    pub page_total: usize,                //总共页码数
    pub page_size: usize,                 //页面显示文章数量
    pub page: usize,                      //当前页面数
}

impl CategoryPageData {
    pub async fn service_category(name: &str, page: usize) -> CategoryPageData {
        let category_result = Category::get_by_name(name).await;
        let categorys = Category::get_all().await;
        println!("{:#?}",&category_result);
        let is_error = match category_result {
            Ok(_) => false,
            Err(_) => true,
        };
        if is_error {
            return CategoryPageData {
                articles: None,
                categorys,
                page_total: 0,
                page_size: 5,
                page: 0,
            };
        };
        let category = category_result.unwrap();
        let category_article = CategoryArticle::get_list_by_category_id(&category.id.unwrap())
            .await
            .unwrap();
        let mut article_id_list = Vec::new();
        for i in category_article {
            article_id_list.push(i.article_id.unwrap())
        }
        article_id_list.reverse();
        let page_total = if article_id_list.len() % 5 == 0 {
            article_id_list.len() / 5
        } else {
            article_id_list.len() / 5 + 1
        };
        let article_page_data = Article::get_article_list_by_id(page, 5, article_id_list).await;
        CategoryPageData {
            articles: Some(article_page_data),
            categorys: categorys,
            page_total: page_total as usize,
            page_size: 5,
            page: page.into(),
        }
    }
}
