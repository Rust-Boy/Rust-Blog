# Rust Blog

> 这是一个由 actix-web 写成的练手 demo，一个博客后端框架
>
> 	前后端分离



部署

```cmd
cargo run
```

部署前请确保 config.json 已经配置好







## 项目描述

```txt
一、前后端分离，耦合低，易与前端组合
二、支持文章的增删改查
三、管理员信息修改、对网站外显修改
四、留言板功能
五、评论功能
```





## API 文档



### 一、页面

| api  | 描述            |
| ---- | --------------- |
| /    | 返回 index.html |





### 二、管理员

| api                                | 描述                     |
| ---------------------------------- | ------------------------ |
| /admin/login                       | 管理员登录               |
| /admin/update                      | 管理员修改账号密码       |
| /admin/addPage                     | 管理员新增文章           |
| /admin/delPage/{id}                | 管理员删除文章           |
| /admin/website/indexMessage/update | 管理员修改首页展示的信息 |





### 三、访客

| api               | 描述         |
| ----------------- | ------------ |
| /vistor/addComit  | 访客添加留言 |
| /vistor/addFriend | 访客添加友链 |





### 四、网站外显信息

| api                          | 描述                   |
| ---------------------------- | ---------------------- |
| /website/friend              | 返回友链列表，json格式 |
| /website/indexMessage/show   | 返回首页展示信息       |
| /website/indexMessage/update | 首页展示信息修改       |





### 五、Blog

| api                    | 描述                                 |
| ---------------------- | ------------------------------------ |
| /blog/commit/{id}/show | 根据id加载对应文章评论，json格式返回 |
| /blog/commit/{id}/add  | 向指定id的文章插入一条评论           |
| /blog/list             | 返回博客文章列表                     |
| /blog/hotPage          | 热门文章，json格式返回               |

