/*
权限验证工具，验证是否登陆；
利用FromRequest请求守卫，获取cookie的login_status;
*/

use rocket::{request::{FromRequest, self, Outcome}, Request, http::Status};
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub struct UserAuth{
    permissions: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cookies = req.cookies();
        let cookie_value = cookies.get_private("login_user_id");
        let status = match cookie_value {
            Some(_) => true,
            None => false,
        };
        if status {
            return Outcome::Success(UserAuth{ permissions:String::from("admin") }) ;
        }else {
            return Outcome::Failure((Status::Unauthorized,()));
        }
    }
}