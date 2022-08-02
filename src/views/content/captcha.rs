use captcha_rs::CaptchaBuilder;
use rocket::{
    get,
    http::{Cookie, CookieJar},
};


//生成验证码 储存在cookie
#[get("/captcha")]
pub async fn index(cookies: &CookieJar<'_>) -> String {
    let captcha = CaptchaBuilder::new()
        .length(6)
        .width(130)
        .height(40)
        .dark_mode(false)
        .complexity(1) // min: 1, max: 10
        .build();
    let mut ck = Cookie::new("content/captcha", captcha.text);
    ck.set_expires(None);
    cookies.add_private(ck);
    captcha.base_img
}
