extern crate r2d2_redis;
use ::redis::{FromRedisValue, RedisError};
use r2d2::{Error, Pool, PooledConnection};
use r2d2_redis::{
    r2d2,
    redis::{self, Commands},
    RedisConnectionManager,
};
use rbatis::DateTimeUtc;
use rocket::response::status;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::ops::DerefMut;
use std::thread;

#[derive(Debug, Deserialize, Serialize)]
pub struct RedisTemplate {
    template: Option<Value>,      //保存模板
    max_time: Option<usize>,      //保存时间 单位 S
    created: Option<DateTimeUtc>, //创建时间
}

//初始化连接池
pub async fn redis_pools() -> Pool<RedisConnectionManager> {
    let manager = RedisConnectionManager::new("rediss://127.0.0.1/").expect("error redis");
    let pool = r2d2::Pool::builder()
        .max_size(35)
        .build(manager)
        .expect("error redis link!");
    pool.clone()
}
impl RedisTemplate {
    pub async fn set_redis_template(url: &str, template_value: RedisTemplate) -> bool {
        let mut conn = redis_pools().await.get().unwrap();
        redis::cmd("SET")
        .arg(url)
        .arg(json!(template_value).to_string())
        .query::<String>(conn.deref_mut())
        .unwrap();
        true
    }
    pub async fn get_redis_template(url: &str) -> RedisTemplate {
        let mut conn = redis_pools().await.get().unwrap();
        let data= redis::cmd("GET")
        .arg(url)
        .query::<String>(conn.deref_mut())
        .unwrap();
        let v = serde_json::from_str::<RedisTemplate>(&data).unwrap();
        v
    }
}
