use serde::{Deserialize, Serialize};

use crate::models::{
    article::Article,
    category::Category,
    tag::{Tag, TagArticle}, comment::Comments,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticlePageData {
    pub article: Option<Article>,         // 文章
    pub categorys: Option<Vec<Category>>, //分类列表
    pub tags: Option<Vec<Option<Tag>>>,           //标签列表
    pub prev_page: bool,                  //是否有旧文章
    pub next_page: bool,                  //是否有新文章
    pub prev_url: Option<String>,         //旧文章url
    pub next_url: Option<String>,         //新文章url
    pub comments:Option<Vec<Comments>>, //文章评论
}

impl ArticlePageData {
    pub async fn service_article(url: &str) -> ArticlePageData {
        let article: Option<Article> = Article::get_article(url).await.unwrap();
        let is_null = match &article {
            Some(_)=>false,
            None => true
        };
        if is_null{
            return ArticlePageData{
                article,
                categorys: None,
                tags: None,
                prev_page: false,
                next_page: false,
                prev_url: Some("Not Found".to_string()),
                next_url: Some("Not Found".to_string()),
                comments: None,
            }
        };
        let article_id = &article.clone().unwrap().id.unwrap();
        let tag_article_list = TagArticle::get_list_by_article_id(article_id).await;
        let categorys = Category::get_all().await;
        let mut tags = Vec::new();
        for i in tag_article_list {
            tags.push(Tag::get_list_by_id(&i.tag_id.unwrap()).await)
        }

        //####   获取下一页相关信息
        let next = Article::get_next_by_id(article_id).await;
        let next_page = match next {
            Ok(_) => true,
            Err(_) => false,
        };
        let next_url = if next_page {
            next.unwrap().url_en
        } else {
            Some("Not Found".to_string())
        };

        //####   获上下一页相关信息
        let prev = Article::get_prev_by_id(article_id).await;
        let prev_page = match prev {
            Ok(_) => true,
            Err(_) => false,
        };
        let prev_url = if prev_page {
            prev.unwrap().url_en
        } else {
            Some("Not Found".to_string())
        };
        let comments = Comments::get_comment_lsit_by_article_id(article_id).await;
        ArticlePageData {
            article,
            categorys,
            tags:Some(tags),
            prev_page,
            next_page,
            prev_url,
            next_url,
            comments,
        }
    }
}
