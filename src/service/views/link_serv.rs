/*
url：/link 对应的服务；
友链
*/

use crate::models::{category::Category, link::Link};
use rbatis::{IPage, IPageRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinkPageData {
    pub links: Vec<Link>,         // 标签列表
    pub categorys: Option<Vec<Category>>, //标签列表
}

impl LinkPageData {
    pub async fn service_link() -> LinkPageData {
        let links = Link::get_all().await.unwrap();
        let categorys = Category::get_all().await;
        LinkPageData { links, categorys }
    }
}
