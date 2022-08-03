use std::fs;
use std::path::Path;
use rocket::fs::NamedFile;
use rocket::get;
use random_number::random;

//获取随机图片
#[get("/random_img")]
pub async fn api_random_img(
) -> Option<NamedFile> {
    let path = Path::new("static/img/random/");
    let mut img_list = Vec::new();
    for entry in fs::read_dir(path).expect("读取目录失败") {
        img_list.push(entry.unwrap().path());
    };
    let max = img_list.len()-1;
    let random = random!(..=max);
    NamedFile::open(img_list.get(random).unwrap()).await.ok()
}
