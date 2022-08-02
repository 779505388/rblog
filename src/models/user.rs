use crate::RB;
use asciis::asc::Asciis;
use rbatis::{crud::CRUD, db::DBExecResult};
use rbatis::{crud_table, Error};
use rocket::serde::json::Json;
use rocket::serde::json::{serde_json::json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::FromForm;
use sha2::{Digest, Sha256};
#[crud_table(table_name:user)]
#[derive(Deserialize, Serialize, Clone, Debug, FromForm)]
pub struct User {
    pub id: Option<usize>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub mail: Option<String>,
    pub mail_hash: Option<String>,
}



impl User {

    pub fn bcrypt(password: String, salt: String) -> String {
        //加密密码--sha3
        let mut password_sha3 = Sha256::new();
        let mut salt_sha3 = Sha256::new();
        let mut result = Sha256::new();
        password_sha3.update(&password);
        salt_sha3.update(&salt);
        let password_result = password_sha3.finalize();
        let salt_result = salt_sha3.finalize();
        let passwrod_salt = format!("{:x}-sd278?><{:x}", password_result, salt_result);
        result.update(&passwrod_salt);
        format!("{:x}", result.finalize())
    }
    /// 登录
    /// ---
    /// @parameter    username    &str
    /// @parameter    password    &str
    /// @return    Result<User, Error>
    /// ---
    pub async fn login_blog(mail: &str, password: &str) -> Value {
        let encode_pwd = User::bcrypt(password.to_string(), "salt".to_string());
        println!("加密后的明文密码：{:#?}", encode_pwd);
        let w = RB
            .new_wrapper()
            .eq("mail", mail)
            .and()
            .eq("password", encode_pwd);
        let login_info = RB.fetch_by_wrapper::<User>(w).await;
        let login_status: bool;
        match login_info {
            Ok(_) => login_status = true,
            Err(_) => login_status = false,
        };
        if login_status {
            return json!({"status":"success","message":"登陆成功！"});
        } else {
            return json!({"status":"error","message":"用户名或密码错误！"});
        }
    }

    /// 获取用户列表
    /// ---
    /// @return  Result<Vec<User>, Error>
    /// ---
    pub async fn get_user_list() -> Result<Vec<User>, Error> {
        RB.fetch_list::<User>().await
    }

    /// 删除用户
    /// ---
    /// @return  Result<usize, Error>
    /// ---
    // pub async fn delete_user(id: &str) -> Result<usize, Error> {
    //     RB.remove_by_column::<User, _>("id", id).await
    // }

    /// 创建用户
    /// ---
    /// @return  Result<DBExecResult, Error>
    /// ---
    pub async fn register_user(user: User) -> bool {
        let username = user.clone().username.unwrap();
        let mail = user.clone().mail.unwrap();
        let mut password = user.clone().password.unwrap();
        let user_username = RB
            .fetch_list_by_column::<User, &str>("username", &[&username])
            .await
            .unwrap();
        let user_mail = RB
            .fetch_list_by_column::<User, &str>("mail", &[&mail])
            .await
            .unwrap();
        if user_mail.len() == 0 && user_username.len() == 0 {
            let digest = md5::compute(&mail);
            password = User::bcrypt(password, "salt".to_string());
            let user_data = User {
                password: Some(password),
                mail_hash: Some(format!("{:x}", digest)),
                ..user
            };
            RB.save::<User>(&user_data, &[]).await;
            // return json!({"status":"success","message":"注册成功！"});
           return true;
        } else {
            // return json!({"status":"error","message":"用户名或邮箱已经存在！"});
           return false;
        };
    }

    //获取博主hash-email
    pub async fn get_hash_email() -> Option<String> {
        let w = RB.new_wrapper().limit(1);
        let r = RB.fetch_by_wrapper::<User>(w).await;
        let mail_hash = match r {
            Ok(i) => i.mail_hash,
            Err(_) => None,
        };
        mail_hash
    }

    //获取博主信息
    pub async fn get_user_by_email(email: &str) -> User {
        let user = RB
            .fetch_by_column::<User, &str>("mail", email)
            .await
            .unwrap();
        user
    }

    //获取博主信息
    pub async fn get_user_by_user_id(user_id: &usize) -> User {
        let user = RB
            .fetch_by_column::<User, &usize>("id", user_id)
            .await
            .unwrap();
        user
    }
    //更新博主信息
    pub async fn update_user(user: User) -> bool {
        let r = RB.update_by_column::<User>("id", &user).await;
        match r {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    pub async fn verify_password(user_id: &usize, password: &str) -> bool {
        let encode_pwd = User::bcrypt(password.to_string(), "salt".to_string());
        println!("加密后的明文密码：{:#?}", encode_pwd);
        let w = RB
            .new_wrapper()
            .eq("id", user_id)
            .and()
            .eq("password", encode_pwd);
        let user_info = RB.fetch_by_wrapper::<User>(w).await;
        match user_info {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
