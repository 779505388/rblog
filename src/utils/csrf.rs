use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rbatis::DateTimeUtc;
use rocket::{
    http::{Cookie, Status},
    request::{self, FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CsrfStatus {
    csrf: bool,
    date: Option<DateTimeUtc>,
}

impl CsrfStatus {
    //加密csrf内容 key储存在cookie中
    pub async fn encrypt_csrf(key: String) -> String {
        let mc = new_magic_crypt!(&key, 192);
        let csrf = json!(CsrfStatus {
            csrf: true,
            date: Some(DateTimeUtc::now())
        })
        .to_string();
        let content = mc.encrypt_str_to_base64(csrf);
        content
    }

    //解密csrf
    pub async fn decrypt_csrf(key: String, csrf: String) -> CsrfStatus {
        let mc = new_magic_crypt!(&key, 192);
        let err_value = json!(CsrfStatus {
            csrf: false,
            date: None
        })
        .to_string();
        let de_csrf = mc.decrypt_base64_to_string(&csrf).unwrap_or(err_value);
        serde_json::from_str::<CsrfStatus>(&de_csrf).unwrap()
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CsrfStatus {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let cookie_value = cookies.get_private("csrf_key");
        let csrf_key = match cookie_value {
            Some(i) => i.value().to_string(),
            None => "None".to_string(),
        };
        let response = req.headers();
        let encrypt = response.get_one("X-CSRFToken").unwrap().to_string();
        let csrf_token = CsrfStatus::decrypt_csrf(csrf_key, encrypt).await;
        println!("{:#?}", &csrf_token);
        if csrf_token.csrf {
            return Outcome::Success(csrf_token);
        } else {
            return Outcome::Failure((Status::Unauthorized,()));
        }
    }
}

//产生key
//验证cookie中是否存在key
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CsrfKey {
    key: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CsrfKey {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let cookie_value = cookies.get_private("csrf_key");
        let status = match cookie_value {
            Some(_) => true,
            None => false,
        };
        if status {
            let uuid = cookie_value.unwrap().value().to_string();
            return Outcome::Success(CsrfKey { key: uuid });
        } else {
            let uuid = Uuid::new_v4().to_string();
            cookies.add_private(Cookie::new("csrf_key", uuid.clone()));
            return Outcome::Success(CsrfKey { key: uuid });
        }
    }
}

// pub  fn csrf_token(key:String)->String{
//     let mc = new_magic_crypt!(&key, 192);
//     let csrf = json!(CsrfStatus {
//         csrf: true,
//         date: Some(DateTimeUtc::now())
//     })
//     .to_string();
//     let content = mc.encrypt_str_to_base64(csrf);
//     content
// }