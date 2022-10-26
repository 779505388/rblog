#[macro_use]
extern crate lazy_static;

use std::env;
use std::sync::Arc;

use crate::service::api::admin::{
    about_api, article_api, category_api, comment_api, link_api, setting_api,
     tag_api,log_api,
};
use crate::service::api::content::random_img;
use rbatis::rbatis::Rbatis;
use rocket::catchers;
use rocket::fs::FileServer;
use rocket::http::Cookie;
use rocket::launch;
use rocket::tokio::sync::Mutex;
use rocket::{fairing::AdHoc, routes};
use rocket_dyn_templates::Template;
use simple_log::LogConfigBuilder;
use uuid::Uuid;
use views::admin::{
    blog_about, blog_article, blog_category, blog_comment, blog_dashboard, blog_link, blog_setting,
    blog_tag,blog_log,
};
use views::content::{
    about, archive, article, captcha, category, comment, home, link, login, 
    register, tag,sitemap,robots,
};
use views::error;
// use fast_log;
use crate::utils::config::Setting;
use dotenv::dotenv;
use utils::info;
mod models;
mod service;
mod utils;
mod views;

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
    static ref CONFIG: Arc<Mutex<Setting>> = Arc::new(Mutex::new(Setting::get_setting()));
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    //初始化连接池
    RB.link(&database_url).await.unwrap();
    let config = LogConfigBuilder::builder()
        .path("./log/tem.log")
        .size(1 * 100)
        .roll_count(10)
        .time_format("%Y-%m-%d %H:%M:%S.%f") //E.g:%H:%M:%S.%f
        .level("warn")
        .output_file()
        .build();
    simple_log::new(config).unwrap();
    let rb = Arc::new(&RB);
    rocket::build()
        .mount(
            "/",
            routes![
                home::index,
                register::index,
                register::post,
                article::index,
                login::index,
                login::post,
                login::login_out,
                category::index,
                tag::index_list,
                tag::index_tag,
                link::index,
                archive::index,
                captcha::index,
                comment::index,
                about::index,
                sitemap::index,
                robots::index,
            ],
        )
        .mount(
            "/admin",
            routes![
                blog_dashboard::index,
                blog_dashboard::info,
                blog_article::list,
                blog_article::post_article,
                blog_article::modify,
                blog_article::post_modify,
                blog_article::add,
                blog_comment::index,
                blog_category::index,
                blog_link::index,
                blog_tag::index,
                blog_about::index,
                blog_setting::index,
                blog_log::index,
            ],
        )
        .mount("/static", FileServer::from("./static"))
        .mount(
            "/api/admin",
            routes![
                article_api::api_article_post,
                article_api::api_article_delete,
                article_api::api_article_put,
                category_api::api_category_get_all,
                category_api::api_category_post,
                category_api::api_category_delete,
                category_api::api_category_put,
                article_api::api_article_get,
                comment_api::api_comment_get_all,
                comment_api::api_comment_delete,
                link_api::api_link_get_all,
                link_api::api_link_post,
                link_api::api_link_put,
                link_api::api_link_delete,
                tag_api::api_tag_get_all,
                tag_api::api_tag_put,
                tag_api::api_tag_delete,
                about_api::api_about_get,
                about_api::api_about_post,
                setting_api::api_setting_get,
                setting_api::api_setting_post_info,
                setting_api::api_setting_post_user,
                log_api::api_log_get,
                log_api::api_log_delete,
            ],
        )
        .mount(
            "/api/content",
            routes![random_img::api_random_img,],
        )
        .register("/", catchers![error::not_found])
        .register("/", catchers![error::server_error])
        .attach(Template::custom(|engines| {
            engines.tera.register_function("blog_info", info::blog_info);
        }))
        .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
            rocket.manage(rb)
        }))
        .attach(AdHoc::on_request("csrf_key", |req, _| {
            Box::pin(async move {
                let cookie = req.cookies();
                let status = match cookie.get_private("csrf_key") {
                    Some(_) => true,
                    None => false,
                };
                if !status {
                    cookie.add_private(Cookie::new("csrf_key", Uuid::new_v4().to_string()));
                };
            })
        }))
}
