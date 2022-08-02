/*
url：/home 对应的服务；
*/

use crate::models::article::Article;
use crate::models::category::Category;
use rbatis::{IPageRequest, IPage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArchivePageData {
    pub articles: Vec<Article>,   // 文章列表
    pub categorys: Option<Vec<Category>>, //标签列表
    pub page_total: usize,          //总共页码数
    pub page_size: usize,           //页面显示文章数量
    pub page: usize,                //当前页面数
}

impl ArchivePageData {
    pub async fn service_archive(page:usize)->ArchivePageData{
        let article_page_data = Article::get_article_list(page, 10).await.unwrap();
        let categorys = Category::get_all().await;
        ArchivePageData {
            articles: article_page_data.get_records().to_vec(),
            categorys:categorys,
            page_total:article_page_data.pages as usize,
            page_size:article_page_data.page_size as usize,
            page:article_page_data.page_no as usize
        }
    }
}
