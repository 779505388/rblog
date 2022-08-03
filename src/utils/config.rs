use rocket::figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct Setting {
    pub avatar_proxyz: Arc<Mutex<String>>,
    pub article_num: Arc<Mutex<usize>>,
    pub blog_name: Arc<Mutex<String>>,
    pub nick_name: Arc<Mutex<String>>,
    pub domain: Arc<Mutex<String>>,
    pub email_hash: Arc<Mutex<String>>,
    pub register: Arc<Mutex<bool>>,
    pub github: Arc<Mutex<String>>,
    pub zhihu: Arc<Mutex<String>>,
    pub telegram: Arc<Mutex<String>>,
    pub email: Arc<Mutex<String>>,
}

impl Setting {
    pub fn get_setting() -> Setting {
        let data = Figment::from(Toml::file("Blog.toml").nested()).select("setting");
        let avatar_proxyz = data
            .find_value("avatar_proxyz")
            .ok()
            .unwrap()
            .into_string()
            .unwrap_or("https://secure.gravatar.com/avatar/".to_string());
        let article_num = data
            .find_value("article_num")
            .ok()
            .unwrap()
            .into_string()
            .unwrap()
            .parse::<usize>()
            .unwrap_or(5);
        let blog_name = data
            .find_value("blog_name")
            .ok()
            .unwrap()
            .into_string()
            .unwrap_or("博客未命名".to_string());
        let nick_name = data
            .find_value("nick_name")
            .ok()
            .unwrap()
            .into_string()
            .unwrap_or("用户未命名".to_string());
        let domain = data
            .find_value("domain")
            .ok()
            .unwrap()
            .into_string()
            .unwrap_or("https:://www.xxxxx.com".to_string());
        let register = data
            .find_value("register")
            .ok()
            .unwrap()
            .into_string()
            .unwrap()
            .parse::<bool>()
            .unwrap_or(false);
        let github = data
            .find_value("github")
            .ok()
            .unwrap()
            .into_string()
            .unwrap_or("".to_string());
        let zhihu = data
            .find_value("zhihu")
            .ok()
            .unwrap()
            .into_string()
            .unwrap_or("".to_string());
        let telegram = data
            .find_value("telegram")
            .ok()
            .unwrap()
            .into_string()
            .unwrap_or("".to_string());
        let email_hash = data
            .find_value("email_hash")
            .ok()
            .unwrap()
            .into_string()
            .unwrap_or("".to_string());
        let email = data
            .find_value("email")
            .ok()
            .unwrap()
            .into_string()
            .unwrap_or("".to_string());
        Setting {
            avatar_proxyz: Arc::new(Mutex::new(avatar_proxyz)),
            blog_name: Arc::new(Mutex::new(blog_name)),
            domain: Arc::new(Mutex::new(domain)),
            email_hash: Arc::new(Mutex::new(email_hash)),
            register: Arc::new(Mutex::new(register)),
            github: Arc::new(Mutex::new(github)),
            zhihu: Arc::new(Mutex::new(zhihu)),
            email: Arc::new(Mutex::new(email)),
            article_num: Arc::new(Mutex::new(article_num)),
            nick_name: Arc::new(Mutex::new(nick_name)),
            telegram: Arc::new(Mutex::new(telegram)),
        }
    }
}
