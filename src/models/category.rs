use crate::RB;
use rbatis::{crud::CRUD, crud_table, db::DBExecResult, DateTimeNative, Error, Page, PageRequest};
#[crud_table(table_name:category)]
#[derive(Clone, Debug)]
pub struct Category {
    pub id: Option<usize>,
    pub name: Option<String>,
}

impl Category {
    //获取所有分类，返回列表
    pub async fn get_all() -> Option<Vec<Category>> {
        let category = RB.fetch_list::<Category>().await;
        match category {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    }

    pub async fn get_by_name(name: &str) -> Result<Category, rbatis::Error> {
        RB.fetch_by_column::<Category, &str>("name", name).await
    }

    //提交新文章分类
    pub async fn commit_category(name: String) -> bool {
        let data = Category {
            id: None,
            name: Some(name),
        };
        let r = RB.save(&data, &[]).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn delete_category_by_id(id: &usize) -> bool {
        let r = RB.remove_by_column::<Category, _>("id", id).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn update_category(id: usize, category: String) -> bool {
        let data = Category {
            id: Some(id),
            name: Some(category),
        };
        let r = RB.update_by_column::<Category>("id", &data).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

#[crud_table(table_name:category_article)]
#[derive(Clone, Debug)]
pub struct CategoryArticle {
    pub id: Option<usize>,
    pub category_id: Option<usize>,
    pub article_id: Option<usize>,
}

impl CategoryArticle {
    pub async fn get_by_article_id(id: &usize) -> Result<CategoryArticle, Error> {
        RB.fetch_by_column::<CategoryArticle, &usize>("article_id", id)
            .await
    }

    pub async fn get_list_by_category_id(id: &usize) -> Result<Vec<CategoryArticle>, Error> {
        RB.fetch_list_by_column::<CategoryArticle, &usize>("category_id", &[id])
            .await
    }

    pub async fn commit_category_name_and_article_id(
        name: &str,
        article_id: Option<usize>,
    ) -> Result<DBExecResult, Error> {
        let category_id = Category::get_by_name(name).await.unwrap().id;
        println!("{:#?}", article_id);
        let data = CategoryArticle {
            id: None,
            category_id,
            article_id,
        };
        let r = RB.save(&data, &[]).await;
        r
    }

    //删除CategoryArticle 的表 通过 article_id
    pub async fn delete_category_article_by_article_id(article_id: &usize) -> bool {
        let category_article = CategoryArticle::get_by_article_id(article_id).await;
        let is_none = match category_article {
            Ok(_) => false,
            Err(_) => true,
        };
        if is_none {
            return true;
        };
        println!("------{:#?}", &category_article);
        let category_id = category_article.unwrap().category_id.unwrap();
        let del = RB
            .remove_by_column::<CategoryArticle, _>("category_id", &category_id)
            .await;
        match del {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    //删除CategoryArticle 的表 通过 category_id
    pub async fn delete_category_article_by_category_id(category_id: &usize) -> bool {
        let r = RB
            .remove_by_column::<CategoryArticle, _>("category_id", category_id)
            .await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub async fn update_category(aricle_id: usize, category: Option<String>) -> bool {
        let category_status = match category {
            Some(_) => true,
            None => false,
        };
        if !category_status {
            return true;
        };
        let name = category.unwrap();
        let category_id = Category::get_by_name(&name).await;
        let category_status = match category_id {
            Ok(_) => true,
            Err(_) => false,
        };
        if !category_status {
            return false;
        };
        let category_id = category_id.unwrap().id;
        let category_data = CategoryArticle {
            id: None,
            article_id: Some(aricle_id),
            category_id,
        };
        let r = RB
            .update_by_column::<CategoryArticle>("article_id", &category_data)
            .await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    //获取文章id列表通过分类id
    pub async fn get_article_id_list_by_category_id(id: &usize) -> Option<Vec<usize>> {
        let r = RB.fetch_list_by_column::<CategoryArticle, &usize>("category_id", &[id])
        .await;
        let status = match r{
            Ok(_) => true,
            Err(_) => false,
        };
        if !status{ return None };
        let mut list = Vec::new();
        let r  = r.unwrap();
        for item in r {
            list.push(item.article_id.unwrap())
        };
        Some(list)
    }
}
