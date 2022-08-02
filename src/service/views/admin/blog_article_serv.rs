use serde::{Serialize, Deserialize};

use crate::models::article::Article;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlogArticle{
    articles:Option<Vec<Article>>,
    csrf_token:Option<String>
}

impl BlogArticle {
    pub async fn article_list()->Option<Vec<Article>>{
        let articles = match Article::get_all_article().await {
            Ok(i) => Some(i),
            Err(_) => None,
        };
        articles
    }

    pub async fn article_modify(id:&usize)->Option<Article>{
        let articles = match Article::get_article_by_id(id).await {
            Ok(i) => i,
            Err(_) => None,
        };
        articles
    }
}

