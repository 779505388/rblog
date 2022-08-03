/*
url：/comment 对应的服务；
*/
use crate::models::comment::Comment;
use md5::compute;
use rbatis::DateTimeNative;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommentData {
    status: String, //success or error
    message: String,
}

impl CommentData {
    pub async fn service_comment(data: Comment,is_admin:bool) -> CommentData {
        let save_data = Comment {
            uuid: Some(Uuid::new_v4().to_string()),
            created: Some(DateTimeNative::now()),
            hash_email: Some(format!("{:x}", compute(data.email.as_ref().unwrap()))),
            is_admin:Some(is_admin),
            is_show:Some(true),
            ..data
        };
        let save = Comment::save_comment(save_data).await;
        let save_status = match save {
            Ok(_) => true,
            Err(_) => false,
        };
        if save_status {
            return CommentData {
                status: "success".to_string(),
                message: "回复成功".to_string(),
            };
        } else {
            return CommentData {
                status: "error".to_string(),
                message: "回复失败".to_string(),
            };
        };
    }
}
