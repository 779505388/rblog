use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

use crate::utils::{auth::UserAuth, csrf::CsrfStatus};
use rocket::{delete, get};
use serde_json::json;
use serde_json::Value;

//按行读取文件
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//获取日志
#[get("/log")]
pub async fn api_log_get(_user_auth: UserAuth, _csrf_status: CsrfStatus) -> Value {
    let text = read_lines("log/tem.log");
    let status = match text {
        Ok(_) => true,
        Err(_) => false,
    };
    if !status {
        return json!({"status":"error","message":"打开Log文件错误！"});
    };
    let mut text_list = Vec::new();

    if let Ok(lines) = text {
        for line in lines {
            if let Ok(data) = line {
                text_list.push(data)
            }
        }
    }
    return json!({"status":"success","message":"打开Log文件成功！","data":text_list});
}

//删除日志
#[delete("/log")]
pub async fn api_log_delete(_user_auth: UserAuth, _csrf_status: CsrfStatus) -> Value {
    let status = fs::write("log/tem.log", "");
    let status = match status {
        Ok(_) => true,
        Err(_) => false,
    };
    if status {
        return json!({"status":"success","message":"删除Log成功！"});
    } else {
        return json!({"status":"error","message":"删除Log文件失败！"});
    }
}
