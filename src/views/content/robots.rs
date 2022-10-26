use rocket::{get, http::ContentType};
// use std::fs;
// use std::path::Path;

use crate::CONFIG;

//robots.txt
#[get("/robots.txt")]
pub async fn index() -> (ContentType, String) {
    let domain = CONFIG
        .try_lock()
        .unwrap()
        .domain
        .try_lock()
        .unwrap()
        .as_str()
        .to_string();
    let robots = format!(
        r#"User-agent: *
Disallow: /login
Disallow: /admin
Disallow: /register
Allow: /
Sitemap: {}/sitemap.xml"#,
        &domain
    );

    (ContentType::Text, robots)
}
