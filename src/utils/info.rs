use dotenv::dotenv;
use rocket_dyn_templates::tera::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, to_value, Value};
use std::{
    collections::{BTreeMap, HashMap},
    env,
};

use crate::CONFIG;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlogInfo {
    pub blog_name: Option<String>,
    pub nick_name: Option<String>,
    pub avatar_proxyz: Option<String>,
    pub email_hash: Option<String>,
    pub github: Option<String>,
    pub zhihu: Option<String>,
    pub email: Option<String>,
}

pub fn blog_info(_args: &HashMap<String, Value>) -> Result<Value> {
    let data = CONFIG.try_lock().unwrap();
    let blog_name = data.blog_name.try_lock().unwrap().to_string();
    let avatar_proxyz = data.avatar_proxyz.try_lock().unwrap().to_string();
    let email_hash = data.email_hash.try_lock().unwrap().to_string();
    let github = data.github.try_lock().unwrap().to_string();
    let zhihu = data.zhihu.try_lock().unwrap().to_string();
    let email = data.email.try_lock().unwrap().to_string();
    let nick_name = data.nick_name.try_lock().unwrap().to_string();
    let info = BlogInfo {
        blog_name: Some(blog_name),
        avatar_proxyz: Some(avatar_proxyz),
        email_hash: Some(email_hash),
        github: Some(github),
        zhihu: Some(zhihu),
        email: Some(email),
        nick_name: Some(nick_name),
    };
    Ok(json!(info))
}
