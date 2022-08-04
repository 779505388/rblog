# rblog

基于rust-rocket构建的blog程序

## 部署程序

1. 修改.env 配置数据库地址。
2. 配置 Rocket.toml、Blog.toml文件。
3. 运行migrations下sql文件，生成表。
4. cargo run --release 生成文件，并移动到文件根目录。

## 实现功能

- [x] 文章增删查改
- [x] 文章标签
- [x] 文章分类
- [x] 回复内容
- [x] csrf 保护
- [x] 博客友链
- [x] 文章归档
- [x] 博客设置
- [x] 登陆注册
- [x] 随机图片
- [x] 后台管理接口API
- [x] sitemap.xml
- [ ] 页面缓存

## 博客图片

![博客截图1](https://raw.githubusercontent.com/779505388/rblog/main/example/001.png)
