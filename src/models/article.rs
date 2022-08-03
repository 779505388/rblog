use crate::RB;
use rbatis::{crud::CRUD, db::DBExecResult};
use rbatis::{crud_table, DateTimeNative, Error, Page, PageRequest};
use serde::{Deserialize, Serialize};
#[crud_table(table_name:article)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Article {
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
    pub views: Option<usize>,
    pub created: Option<DateTimeNative>,
    pub modified: Option<DateTimeNative>,
}

impl Article {
    // 获取文章列表
    // page：页数
    //num：每页显示文章数量
    pub async fn get_article_list(page: usize, num: usize) -> Result<Page<Article>, Error> {
        let req = PageRequest::new(page as u64, num as u64);
        let wraper = RB.new_wrapper().order_by(false, &["id"]);
        let r = RB.fetch_page_by_wrapper::<Article>(wraper, &req).await;
        println!("{:#?}", &r);
        r
    }

    //通过[id]获取文章
    pub async fn get_article_list_by_id(
        page: usize,
        num: usize,
        id_list: Vec<usize>,
    ) -> Vec<Article> {
        let start = (page - 1) * num;
        let id_len = id_list.len() as usize;
        let end = if id_len < &start + num {
            id_len
        } else {
            &start + num
        };
        let list_start: usize = start.try_into().unwrap();
        let list_end: usize = end.try_into().unwrap();
        let mut article_list = Vec::new();
        for i in &id_list[list_start..list_end] {
            article_list.push(
                RB.fetch_by_column::<Article, &usize>("id", i)
                    .await
                    .unwrap(),
            );
        }
        article_list
    }

    // 获取文章
    // url：通过url_en获取
    pub async fn get_article(url: &str) -> Result<Option<Article>, Error> {
        let data: Result<Option<Article>, Error> = RB.fetch_by_column("url_en", url).await;
        data
    }

    // 获取文章
    // url：通过id获取
    pub async fn get_article_by_id(id: &usize) -> Result<Option<Article>, Error> {
        let data: Result<Option<Article>, Error> = RB.fetch_by_column("id", id).await;
        data
    }

    //获取下一篇文章
    pub async fn get_next_by_id(article_id: &usize) -> Result<Article, Error> {
        let w = RB
            .new_wrapper()
            .gt("id", article_id)
            .order_by(true, &["id"])
            .limit(1);
        let r = RB.fetch_by_wrapper::<Article>(w).await;
        r
    }

    //获取上一篇文章
    pub async fn get_prev_by_id(article_id: &usize) -> Result<Article, Error> {
        let w = RB
            .new_wrapper()
            .lt("id", article_id)
            .order_by(false, &["id"])
            .limit(1);
        let r = RB.fetch_by_wrapper::<Article>(w).await;
        r
    }

    //获取最近文章
    pub async fn get_recent_article() -> Result<Vec<Article>, Error> {
        let w = RB.new_wrapper().order_by(false, &["id"]).limit(7);
        let r = RB.fetch_list_by_wrapper::<Article>(w).await;
        r
    }

    //获取最近文章
    pub async fn get_all_article() -> Result<Vec<Article>, Error> {
        let w = RB.new_wrapper().order_by(false, &["id"]);
        let r = RB.fetch_list_by_wrapper::<Article>(w).await;
        r
    }

    //提交新文章
    pub async fn commit_article(data: Article) -> Result<DBExecResult, Error> {
        let r = RB.save(&data, &[]).await;
        r
    }

    pub async fn delete_article_by_id(id: &usize) -> bool {
        let r = RB.remove_by_column::<Article, _>("id", id).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn update_article(data: &Article) -> bool {
        let status = RB.update_by_column::<Article>("id", data).await;
        match status {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    //修改文章分类
    pub async fn update_article_category(list_id: Vec<usize>, category: String) -> bool {
        for id in list_id {
            let data = Article {
                id: Some(id),
                title: None,
                brief: None,
                user_id: None,
                url_en: None,
                text: None,
                template: None,
                image_url: None,
                like_count: None,
                category: Some(category.clone()),
                views: None,
                created: None,
                modified: None,
            };
            let r = RB.update_by_column::<Article>("id", &data).await;
            let status = match r {
                Ok(_) => true,
                Err(_) => false,
            };
            if !status {
                return false;
            };
        }
    return true;
    }
}
