use crate::RB;
use rbatis::{crud::CRUD, crud_table, db::DBExecResult, Error};
#[crud_table(table_name:tag)]
#[derive(Clone, Debug)]
pub struct Tag {
    pub id: Option<usize>,
    pub name: Option<String>,
}

impl Tag {
    //获取所有标签
    pub async fn get_all() -> Result<Vec<Tag>, rbatis::Error> {
        RB.fetch_list::<Tag>().await
    }

    //通过Id获取Tag
    pub async fn get_list_by_id(tag_id: &usize) -> Option<Tag> {
        let r = RB.fetch_by_column::<Tag, &usize>("id", tag_id).await;
        match r {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    }

    //通过name获取Tag
    pub async fn get_by_name(name: &str) -> Tag {
        RB.fetch_by_column::<Tag, &str>("name", name).await.unwrap()
    }

    //通过name获取，判断是否存在
    pub async fn get_tag_exist(name: &str) -> bool {
        let r = RB.fetch_by_column::<Tag, &str>("name", name).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    //提交
    pub async fn commit_tag(name: String) -> Result<DBExecResult, Error> {
        let tag = Tag {
            id: None,
            name: Some(name),
        };
        RB.save(&tag, &[]).await
    }

    //修改tag
    pub async fn update_tag(data: Tag) -> bool {
        let r = RB.update_by_column::<Tag>("id", &data).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    //删除tag
    pub async fn delete_tag(tag_id: &usize) -> bool {
        let r = RB.remove_by_column::<Tag, _>("id", tag_id).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

#[crud_table(table_name:tag_article)]
#[derive(Clone, Debug)]
pub struct TagArticle {
    pub id: Option<usize>,
    pub tag_id: Option<usize>,
    pub article_id: Option<usize>,
}

impl TagArticle {
    //通过文章id获取标签id
    pub async fn get_list_by_article_id(article_id: &usize) -> Vec<TagArticle> {
        let data = RB
            .fetch_list_by_column::<TagArticle, &usize>("article_id", &[article_id])
            .await
            .unwrap();
        data
    }

    //通过文章id获取标签id
    pub async fn get_tag_list_by_article_id(article_id: &usize) -> Result<Vec<TagArticle>, Error> {
        let data = RB
            .fetch_list_by_column::<TagArticle, &usize>("article_id", &[article_id])
            .await;
        data
    }

    //通过标签id获取文章id
    pub async fn get_list_by_tag_id(tag_id: &usize) -> Vec<TagArticle> {
        let data = RB
            .fetch_list_by_column::<TagArticle, &usize>("tag_id", &[tag_id])
            .await
            .unwrap();
        data
    }

    //提交标签与文章关系
    pub async fn commit_tag_article(article_id: Option<usize>, tags: Option<Vec<String>>) -> bool {
        let is_none = match tags {
            Some(_) => false,
            None => true,
        };
        if is_none {
            return true;
        };
        for tag in tags.unwrap() {
            let status = Tag::get_tag_exist(&tag).await;
            if status {
                let tag_id = Tag::get_by_name(&tag).await.id;
                let data = TagArticle {
                    id: None,
                    tag_id,
                    article_id,
                };
                let _r = RB.save(&data, &[]).await;
            } else {
                let tag_commit = Tag::commit_tag(tag.clone()).await;
                let commit_status = match tag_commit {
                    Ok(_) => true,
                    Err(_) => false,
                };
                if commit_status {
                    let tag_id = Tag::get_by_name(&tag).await.id;
                    let data = TagArticle {
                        id: None,
                        tag_id,
                        article_id,
                    };
                    let _r = RB.save(&data, &[]).await;
                } else {
                    return false;
                };
            };
        }
        return true;
    }

    //删除通过文章id
    pub async fn delete_tag_article_by_article_id(article_id: &usize) -> bool {
        let tag_article = TagArticle::get_tag_list_by_article_id(article_id).await;
        let is_none = match tag_article {
            Ok(_) => false,
            Err(_) => true,
        };
        if is_none {
            return true;
        };
        let mut id_list = vec![];
        for item in tag_article.unwrap() {
            id_list.push(item.id.unwrap())
        }
        let del = RB
            .remove_batch_by_column::<TagArticle, _>("id", &id_list)
            .await;
        match del {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    //删除通过标签id
    pub async fn delete_tag_article_by_tag_id(tag_id: &usize) -> bool {
        let tag_article = RB.remove_by_column::<TagArticle, _>("tag_id", tag_id).await;
        match tag_article {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    //更新文章标签
    pub async fn update_tags(article_id: usize, tag_list: Option<Vec<String>>) -> bool {
        let status = match tag_list {
            Some(_) => true,
            None => false,
        };
        if !status {
            return true;
        };
        let status = TagArticle::delete_tag_article_by_article_id(&article_id).await;
        if !status {
            return false;
        };
        for tag in tag_list.unwrap() {
            let status = Tag::get_tag_exist(&tag).await;
            if status {
                let tag_id = Tag::get_by_name(&tag).await.id;
                let data = TagArticle {
                    id: None,
                    tag_id,
                    article_id: Some(article_id),
                };
                let _r = RB.save(&data, &[]).await;
            } else {
                let tag_commit = Tag::commit_tag(tag.clone()).await;
                let commit_status = match tag_commit {
                    Ok(_) => true,
                    Err(_) => false,
                };
                if commit_status {
                    let tag_id = Tag::get_by_name(&tag).await.id;
                    let data = TagArticle {
                        id: None,
                        tag_id,
                        article_id: Some(article_id),
                    };
                    let _r = RB.save(&data, &[]).await;
                } else {
                    return false;
                };
            };
        }
        return true;
    }
}


