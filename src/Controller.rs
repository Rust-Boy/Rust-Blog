#[allow(warnings)]

// 管理员的操作
#[allow(warnings)]
pub mod admin {
    use actix_web::{HttpResponse, get, web};
    use crate::Model::model::{Admin, DB};
    use crate::Tools;
    use crate::Tools::Jwt::{isLogin, jwtCreate};

    // 拓展 Model 中的结构体的功能
    pub mod AssistStruct {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Deserialize, Serialize)]
        pub struct Admin {
            pub(crate) jwt: String,
            pub(crate) username: String,
            pub(crate) password: String
        }

        // 删除文章的请求
        #[derive(Debug, Deserialize, Serialize)]
        pub struct DelPage {
            pub(crate) jwt: String,
            pub(crate) id: String
        }

        // 新增文章的请求
        #[derive(Debug, Deserialize, Serialize)]
        pub struct page {
            pub(crate) jwt: String,
            pub(crate) page: crate::Model::model::Page
        }

        // 首页展示的信息
        #[derive(Debug, Deserialize, Serialize)]
        pub struct IndexPage {
            pub(crate) jwt: String,
            pub(crate) indexMessage: crate::Model::model::IndexPage
        }
    }

    // 管理员登录
    #[get("/admin/login")]
    pub fn adminLogin(admin: web::Query<Admin>) -> HttpResponse {
        let admin = admin.0;

        let username = admin.username.clone();
        let password = admin.password.clone();

        // 是否重复登录
        if isLogin(Tools::Jwt::jwt(Admin {
            username: username.clone(),
            password: password.clone()
        })) {
            return HttpResponse::Ok().body("already login")
        }

        // fn login(mut pc: PooledConn, admin: Admin) -> bool
        if DB::login(DB::new(), admin) {
            jwtCreate(Admin {
                username,
                password
            });
            return HttpResponse::Ok().body("login success")
        }
        HttpResponse::Ok().body("login failed")
    }

    // 管理员新增文章
    #[get("/admin/addPage")]
    pub fn adminAddPage(addPage: web::Query<AssistStruct::page>) -> HttpResponse {
        let addPage = addPage.0;
        if !isLogin(addPage.jwt) {
            return HttpResponse::Ok().body("please login")
        }

        if DB::addPage(DB::new(), addPage.page) {
            return HttpResponse::Ok().body("add page success")
        }
        HttpResponse::Ok().body("add page failed")
    }

    // 管理员根据文章id删除文章
    #[get("/admin/delPage/{id}")]
    pub fn adminDelPageId(delPage: web::Query<AssistStruct::DelPage>) -> HttpResponse {
        let delPage = delPage.0;

        if !isLogin(delPage.jwt) {
            return HttpResponse::Ok().body("please login")
        }
        if DB::delPage(DB::new(), delPage.id) {
            return HttpResponse::Ok().body("del page success")
        }
        HttpResponse::Ok().body("del page failed")
    }

    // 管理员信息修改
    #[get("/admin/update")]
    pub fn adminUpdate(admin: web::Query<AssistStruct::Admin>) -> HttpResponse {
        let admin = admin.0;

        // 是否已登录
        if !isLogin(admin.jwt.clone()) {
            return HttpResponse::Ok().body("please login")
        }

        if DB::updateAdmin(DB::new(), Admin {
            username: admin.username,
            password: admin.password
        }) {
            return HttpResponse::Ok().body("update success")
        }

        HttpResponse::Ok().body("")
    }

    // 管理员修改首页展示的信息
    #[get("/admin/website/indexMessage/update")]
    pub fn websiteIndexMessageUpdate(indexMessage: web::Query<AssistStruct::IndexPage>) -> HttpResponse {
        if !isLogin(indexMessage.0.jwt) {
            return HttpResponse::Ok().body("please login")
        }

        let indexMessage = indexMessage.0.indexMessage;

        if DB::websiteIndexMessageUpdate( DB::new(),indexMessage) {
            return HttpResponse::Ok().body("update success")
        }

        HttpResponse::Ok().body("update failed")
    }
}

#[allow(warnings)]
// 访客的操作
pub mod vistor {
    use actix_web::{HttpResponse, get, web};
    use crate::Model;
    use crate::Model::model::{Admin, DB, FriendLink, record};
    use crate::Tools::Jwt::{isLogin, jwtCreate};

    // 访客添加留言
    #[get("/vistor/addComit")]
    pub fn vistorAddCommit(record: web::Query<record>) -> HttpResponse {
        if DB::addRecord(DB::new(), record.0) {
            return HttpResponse::Ok().body("add commit success")
        }

        HttpResponse::Ok().body("add commit failed")
    }

    // 访客添加友链
    #[get("/vistor/addFriend")]
    pub fn vistorAddFriend(fl: web::Query<FriendLink>) -> HttpResponse {
        if !DB::addFriend(DB::new(), fl.0) {
            return HttpResponse::Ok().body("add success")
        }
        HttpResponse::Ok().body("add failed")
    }
}

#[allow(warnings)]
// 对网站外显信息进行操作
pub mod website {
    use actix_web::{HttpResponse, get, web};
    use crate::Model::model::DB;
    use crate::Tools::Jwt::isLogin;

    pub mod AssistStruct {
        use serde::{Serialize, Deserialize};

        // 首页展示信息修改
        #[derive(Debug, Deserialize, Serialize)]
        pub struct IndexPageUpdate {
            pub(crate) jwt: String,
            pub(crate) indexMessage: crate::Model::model::IndexPage
        }
    }

    // 返回友链列表，json格式
    #[get("/website/friend")]
    pub fn websiteFriend() -> HttpResponse {
        HttpResponse::Ok().json(
            DB::friendLink(DB::new())
        )
    }

    // 返回首页展示信息
    #[get("/website/indexMessage/show")]
    pub fn websiteIndexMessageShow() -> HttpResponse {
        HttpResponse::Ok().json(DB::websiteIndexMessageShow(DB::new()))
    }

    // 首页展示信息修改
    #[get("/website/indexMessage/update")]
    pub fn websiteIndexMessageUpdate(iMsg: web::Query<AssistStruct::IndexPageUpdate>) -> HttpResponse {
        if !isLogin(iMsg.0.jwt) {
            return HttpResponse::Ok().body("please login")
        }

        if DB::websiteIndexMessageUpdate(DB::new(), iMsg.0.indexMessage) {
            return HttpResponse::Ok().body("update success")
        }

        HttpResponse::Ok().body("update failed")
    }
}

#[allow(warnings)]
// 对 blog 的操作
pub mod blog {
    use actix_web::{HttpResponse, get, web};
    use crate::Model::model::{commitID, DB};

    // 根据id加载对应文章评论，json格式返回
    #[get("/blog/commit/{id}/show")]
    pub fn blogCommitIdShow(web::Path(id): web::Path<String>) -> HttpResponse {
        HttpResponse::Ok().json(
            DB::getPageCommitById(DB::new(), id)
        )
    }

    // 向指定id的文章插入一条评论
    #[get("/blog/commit/{id}/add")]
    pub fn blogCommitIdAdd(commit: web::Query<commitID>) -> HttpResponse {
        if DB::addCommit(DB::new(), commit.0) {
            return HttpResponse::Ok().body("add success")
        }
        HttpResponse::Ok().body("add failed")
    }

    // 返回博客文章列表
    #[get("/blog/list")]
    pub fn blogList() -> HttpResponse {
        HttpResponse::Ok().json(DB::selectPages(DB::new()))
    }

    // 热门文章
    #[get("/blog/hotPage")]
    pub fn hotPage() -> HttpResponse {
        HttpResponse::Ok().json(
            DB::hotPage(DB::new())
        )
    }
}

// 对页面的操作，建议页面路由都放在这里
#[allow(warnings)]
pub mod page {
    use std::fs;
    use actix_web::{HttpResponse, get};

    #[get("/")]
    pub fn index() -> HttpResponse {
        HttpResponse::Ok().body(fs::read_to_string("src/static/index.html").unwrap())
    }
}