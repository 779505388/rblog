use serde::{Deserialize, Serialize};
use psutil::disk;
use  psutil::memory::virtual_memory;
use crate::models::{article::Article, comment::Comment};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DashboardInfo {
articles:Option<Vec<Article>>,
comments:Option<Vec<Comment>>,
free_disk:u64,
total_disk:u64,
free_mem:u64,
total_mem:u64,
}

impl DashboardInfo {
    pub async fn service_dashboard()-> DashboardInfo {
        let articles = match Article::get_recent_article().await {
            Ok(i) => Some(i),
            Err(_) => None,
        };
        let comments = match Comment::get_recent_comment().await {
            Ok(i) => Some(i),
            Err(_) => None,
        };
        let disk_space = disk::disk_usage(Path::new("/")).unwrap();
        let total_disk = disk_space.total();
        let free_disk = disk_space.free();
        let memory = virtual_memory().unwrap();
        let free_mem = memory.available();
        let total_mem = memory.total();
        DashboardInfo {
            articles,
            comments,
            total_disk,
            free_disk,
            free_mem,
            total_mem
        }

    }
}