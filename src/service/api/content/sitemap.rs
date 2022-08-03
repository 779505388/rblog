// use rocket::fs::NamedFile;
use rocket::{get, http::ContentType};
// use std::fs;
// use std::path::Path;

use crate::{models::article::Article, CONFIG};

//生成站点地图
#[get("/sitemap.xml")]
pub async fn api_sitemap_get() -> (ContentType,String) {
    let articles = Article::get_all().await;
    let domain = CONFIG
        .try_lock()
        .unwrap()
        .domain
        .try_lock()
        .unwrap()
        .as_str()
        .to_string();
    let mut sitemap = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
    <urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
    );
    for item in articles {
        let text = format!(
            r#"
        <url>
        <loc>{}/{}</loc>
        <lastmod>{}</lastmod>
        <changefreq>weekly</changefreq>
      <priority>0.8</priority>
      </url>
        "#,
            &domain,
            item.url_en.unwrap(),
            item.modified.unwrap()
        );
        sitemap += &text;
    }
    sitemap += "</urlset>";
    (ContentType::XML, sitemap)
}
