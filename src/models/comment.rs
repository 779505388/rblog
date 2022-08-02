use rbatis::{crud_table, DateTimeNative, crud::CRUD, db::DBExecResult};
use serde::{Deserialize, Serialize};
use rbatis::Error;
use crate::RB;
#[crud_table(table_name:comment)]
#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct Comment {
    pub id: Option<usize>,
    pub user_name: Option<String>,
    pub article_id: Option<usize>,
    pub parent_id: Option<usize>,
    pub parent_uuid: Option<String>,
    pub parent_name: Option<String>,
    pub is_admin: Option<bool>,
    pub agent: Option<String>,
    pub is_show: Option<bool>,
    pub uuid: Option<String>,
    pub text: Option<String>,
    pub web_site: Option<String>,
    pub email: Option<String>,
    pub hash_email: Option<String>,
    pub is_review: bool,
    pub created: Option<DateTimeNative>,
}

impl Comment {
    pub async fn save_comment(comment_data:Comment)-> Result<DBExecResult,Error> {
    let save = RB.save(&comment_data,&[]).await;
    save
}

    //获取最近评论
    pub async fn get_recent_comment() -> Result<Vec<Comment>,Error> {
        let w = RB.new_wrapper()
        .order_by(false, &["id"])
        .limit(7);
        let r = RB.fetch_list_by_wrapper::<Comment>(w).await;
        r
    }

     //获取最近评论
     pub async fn get_comment_all() -> Result<Vec<Comment>,Error> {
        let w = RB.new_wrapper()
        .order_by(false, &["id"]);
        let r = RB.fetch_list_by_wrapper::<Comment>(w).await;
        r
    }

    //删除评论
    pub async fn delete_comment_list(data:&Vec<usize>)->bool{
        for id in data{
            let r = RB.remove_by_column::<Comment,_>("id", id).await;
            let status = match r {
                Ok(_) => true,
                Err(_) => false,
            };
            if !status{
                return false;
            };
        };
        true
    }
}


#[crud_table(table_name:comments)]
#[derive(Clone, Debug)]
pub struct Comments{
   pub comment:Option<Comment>,
   pub child_comments:Option<Vec<Comment>>
}

impl Comments {
    pub async fn get_comment_lsit_by_article_id(article_id:&usize)->Option<Vec<Comments>>{
        let mut vec_comment:Vec<Comments> = Vec::new();
        let parent_commemts = RB.fetch_list_by_column::<Comment,&usize>("article_id",&[article_id]).await;
        let is_null = match &parent_commemts {
            Ok(_)=>false,
            Err(_)=>true,};
        if is_null{
            println!("null--{}",article_id);
            return Some(vec_comment);
        };
        for item in parent_commemts.unwrap(){
            let child_comments = RB.fetch_list_by_column::<Comment,&usize>("parent_id",&[&item.id.unwrap()]).await;
            let child_comment_s = Comments{
                comment:Some(item),
                child_comments:Some(child_comments.unwrap())
            };
            vec_comment.push(child_comment_s);
        };
       Some(vec_comment)
    }
}