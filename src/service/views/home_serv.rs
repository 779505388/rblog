/*
url：/archive 对应的服务；
归档记录
*/

use crate::models::article::Article;
use crate::models::category::Category;
use rbatis::IPage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HomePageData {
    pub articles: Vec<Article>,   // 文章列表
    pub categorys: Option<Vec<Category>>, //标签列表
    pub page_total: usize,          //总共页码数
    pub page_size: usize,           //页面显示文章数量
    pub page: usize,                //当前页面数
}

impl HomePageData {
    pub async fn service_home(page:usize)->HomePageData{
        let article_page_data = Article::get_article_list(page, 5).await.unwrap();
        let categorys = Category::get_all().await;
        HomePageData {
            articles: article_page_data.get_records().to_vec(),
            categorys:categorys,
            page_total:article_page_data.pages as usize,
            page_size:article_page_data.page_size as usize,
            page:article_page_data.page_no as usize
        }
    }
}
