#[allow(warnings)]

use std::fmt::Result;
use std::fs;
use mysql::{Pool, Opts, from_row, PooledConn};
use mysql::prelude::Queryable;
use json;

pub mod model {
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Config {
        pub(crate) website: Website,
        pub(crate) mysql: Mysql
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Mysql {
        pub(crate) username: String,
        pub(crate) password: String,
        pub(crate) address: String,
        pub(crate) port: String,
        pub(crate) dbname: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Website {
        pub(crate) address: String,
        pub(crate) port: String,
        pub(crate) workers: String
    }

    // 站内所有文章信息
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Pages {
        pub(crate) pages: Vec<Page>
    }

    // 单篇文章
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Page {
        pub(crate) title: String,
        pub(crate) id: String,
        pub(crate) author: String,
        pub(crate) content: String,
        pub(crate) date: String,
        pub(crate) create_time: String,
        pub(crate) cove_image: String,
        pub(crate) summary: String,
        pub(crate) read_number: String,
        pub(crate) thumb_up_number: String,
        pub(crate) comment_number: String,
        pub(crate) tick: String
    }

    // 指定id的文章的评论，每篇文章的评论单独成表
    #[derive(Debug, Deserialize, Serialize)]
    pub struct CommitID {
        pub(crate) commitID: Vec<commitID>
    }

    // 一条评论
    #[derive(Debug, Deserialize, Serialize)]
    pub struct commitID {
        pub(crate) id: String,
        pub(crate) qq: String,
        pub(crate) name: String,
        pub(crate) content: String,
        pub(crate) date: String
    }

    // 留言结构
    #[derive(Debug, Deserialize, Serialize)]
    pub struct record {
        pub(crate) qq: String,
        pub(crate) name: String,
        pub(crate) content: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Record {
        pub(crate) record: Vec<record>
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct RecordPool {
        qq: String,
        name: String,
        content: String
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Data;

    // 数据库操作类
    #[derive(Debug, Deserialize, Serialize)]
    pub struct DB;

    // 管理员
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Admin {
        pub(crate) username: String,
        pub(crate) password: String
    }

    // 一条友链
    #[derive(Debug, Deserialize, Serialize)]
    pub struct FriendLink {
        pub(crate) qq: String,
        pub(crate) name: String,
        pub(crate) url: String,
        pub(crate) descr: String
    }

    // 友链列表篇
    #[derive(Debug, Deserialize, Serialize)]
    pub struct FriendLinkS {
        pub(crate) friendLink: Vec<FriendLink>
    }

    // 首页展示的信息
    #[derive(Debug, Deserialize, Serialize)]
    pub struct IndexPage {
        pub(crate) name: String,
        pub(crate) descr: String
    }
}

pub mod Impl {
    use std::fs;
    use mysql::{from_row, Opts, Pool, PooledConn};
    use mysql::prelude::Queryable;
    use crate::Model::model::{Admin, CommitID, commitID, Config, Data, DB, FriendLink, FriendLinkS, IndexPage, Mysql, Page, Pages, Record, record, Website};

    impl Config {
        pub fn new() -> Config {
            let Json = fs::read_to_string("src/config/mysql.json").unwrap();
            let Json = json::parse(&Json).unwrap();
            Config {
                website: Website {
                    address: Json["website"]["address"].to_string(),
                    port: Json["website"]["port"].to_string(),
                    workers: Json["website"]["workers"].to_string()
                },
                mysql: Mysql {
                    username: Json["mysql"]["username"].to_string(),
                    password: Json["mysql"]["password"].to_string(),
                    address: Json["mysql"]["address"].to_string(),
                    port: Json["mysql"]["port"].to_string(),
                    dbname: Json["mysql"]["dbname"].to_string()
                }
            }
        }
    }

    impl Data {
        // 根据 id 返回 文章所有评论
        fn getCommitByID(id: String) -> Record {
            Record {
                record: Vec::new()
            }
        }
    }

    impl DB {
        // 返回一个数据库连接对象
        pub fn new() -> PooledConn {
            let config = Config::new();

            let url = format!("mysql://{}:{}@{}:{}/{}",
                              config.mysql.username,
                              config.mysql.password,
                              config.mysql.address,
                              config.mysql.port,
                              config.mysql.dbname
            );

            let url = Opts::from_url(&url).unwrap();
            let pool = Pool::new(url).unwrap();
            pool.get_conn().unwrap()
        }

        // 取所有文章的信息
        pub fn selectPages(mut pc: PooledConn) -> Pages {
            let mut pages = Pages { pages: Vec::new() };
            let sql = format!("select * from Pages");

            pc.query_iter(sql).unwrap()
                .for_each(|row| {
                    let r: (String, String, String, String, String, String, String, String, String, String, String, String) = from_row(row.unwrap());
                    let page = Page {
                        title: r.0,
                        id: r.1,
                        author: r.2,
                        content: r.3,
                        date: r.4,
                        create_time: r.5,
                        cove_image: r.6,
                        summary: r.7,
                        read_number: r.8,
                        thumb_up_number: r.9,
                        comment_number: r.10,
                        tick: r.11
                    };
                    pages.pages.push(page)
                });

            pages
        }

        // 取指定id文章的信息
        pub fn getPageById(mut pc: PooledConn, id: String) -> Page {
            let sql = format!("select * from Pages where id={}", id);

            let mut title = String::new();
            let mut id = String::new();
            let mut author = String::new();
            let mut content = String::new();
            let mut date = String::new();
            let mut create_time = String::new();
            let mut cove_image = String::new();
            let mut summary = String::new();
            let mut read_number = String::new();
            let mut thumb_up_number = String::new();
            let mut comment_number = String::new();
            let mut tick = String::new();

            pc.query_iter(sql).unwrap()
                .for_each(|row| {
                    let r: (String, String, String, String, String, String, String, String, String, String, String, String) = from_row(row.unwrap());

                    title = r.0;
                    id = r.1;
                    author = r.2;
                    content = r.3;
                    date = r.4;
                    create_time = r.5;
                    cove_image = r.6;
                    summary = r.7;
                    read_number = r.8;
                    thumb_up_number = r.9;
                    comment_number = r.10;
                    tick = r.11;
                });

            Page {
                title,
                id,
                author,
                content,
                date,
                create_time,
                cove_image,
                summary,
                read_number,
                thumb_up_number,
                comment_number,
                tick
            }
        }

        // 取指定id的文章的所有评论
        pub fn getPageCommitById(mut pc: PooledConn, id: String) -> CommitID {
            let mut _commitID = CommitID { commitID: Vec::new() };
            let sql = format!("select * from Commit{}", id);

            pc.query_iter(sql).unwrap()
                .for_each(|row| {
                    let r: (String, String, String, String, String) = from_row(row.unwrap());

                    _commitID.commitID.push(commitID {
                        id: r.0,
                        qq: r.1,
                        name: r.2,
                        content: r.3,
                        date: r.4
                    })
                });

            _commitID
        }

        // 新增一篇文章
        pub fn addPage(mut pc: PooledConn, page: Page) -> bool {
            let mut sql = "select count(*) from Pages".to_string();
            let mut count: u8 = pc.query_first(sql).unwrap().unwrap();
            if count == 0 {
                sql = format!(
                    "insert into Pages(title, id, author, date, date_time, create_time, cove_image, summary, read_number, thumb_up_number, comment_number, tick) values('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
                    page.title,
                    String::from("0"),
                    page.author,
                    page.content,
                    page.date,
                    page.create_time,
                    page.cove_image,
                    page.summary,
                    page.read_number,
                    page.thumb_up_number,
                    page.comment_number,
                    page.tick
                );

                let mut res: i32 = pc.query_first(sql).unwrap().unwrap();
                return true;
            }

            sql = format!(
                "insert into Pages(title, id, author, date, date_time, create_time, cove_image, summary, read_number, thumb_up_number, comment_number, tick) values('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
                page.title,
                format!("{}", count + 1),
                page.author,
                page.content,
                page.date,
                page.create_time,
                page.cove_image,
                page.summary,
                page.read_number,
                page.thumb_up_number,
                page.comment_number,
                page.tick
            );
            let _ = pc.query_first::<i32, String>(sql).unwrap();
            false
        }

        // 根据id删除文章，本质是变成相反数
        pub fn delPage(mut pc: PooledConn, id: String) -> bool {
            let ID = format!("-{}", id);
            let sql = format!("update Pages set id={} where id={}", ID, id);

            let mut res: i32 = pc.query_first(sql).unwrap().unwrap();

            if res > 0 {
                return true
            }
            false
        }

        // admin修改密码
        pub fn updateAdmin(mut pc: PooledConn, admin: Admin) -> bool {
            let sql = format!("update User set password={} where username={}",
                              admin.password,
                              admin.username
            );
            let _ = pc.query_first::<i32, String>(sql).unwrap();
            true
        }

        // 用户登录
        pub fn login(mut pc: PooledConn, admin: Admin) -> bool {
            let sql = format!("select count(*) from User where username={} and password={}",
                              admin.username,
                              admin.password
            );

            let count = pc.query_first::<i32, String>(sql.clone()).unwrap().unwrap();
            if count == 1 {
                let _ = pc.query_first::<i32, String>(sql).unwrap();
            } else {
                return false
            }
            true
        }

        // 向指定id的文章写入一条评论（假定表已存在，这是小bug)
        pub fn addCommit(mut pc: PooledConn, commit: commitID) -> bool {
            let sql = format!("insert into Commit{}(id,qq,name,content,date) values('{}','{}','{}','{}','{}')",
                              commit.id,
                              commit.id,
                              commit.qq,
                              commit.name,
                              commit.content,
                              commit.date
            );
            // 一条评论
            // #[derive(Debug)]
            // struct commitID {
            //     id: String,
            //     qq: String,
            //     name: String,
            //     content: String,
            //     date: String
            // }
            let _ = pc.query_first::<i32, String>(sql).unwrap();
            true
        }

        // 添加留言
        pub fn addRecord(mut pc: PooledConn, _record: record) -> bool {
            // 留言结构
            // #[derive(Debug)]
            // struct record {
            //     qq: String,
            //     name: String,
            //     content: String
            // }

            // create table Record(
            //     qq text not null,
            //     name text not null,
            //     content text not null
            // );
            let sql = format!("insert into Record(qq,name,content) values('{}','{}','{}')",
                              _record.qq,
                              _record.name,
                              _record.content
            );
            let res: Option<i32> = pc.query_first::<i32, String>(sql).unwrap();
            match res {
                Some(num) => {
                    if num == 1 {
                        return true
                    }
                },
                _ => return false
            }
            true
        }

        // 删除留言
        pub fn delRecord(mut pc: PooledConn, _record: record) -> bool {
            let sql = format!("delete from Record where qq={}", _record.qq);
            let res: Option<i32> = pc.query_first::<i32, String>(sql).unwrap();
            match res {
                Some(num) => {
                    if num == 1 {
                        return true
                    } else {
                        return false
                    }
                },
                _ => return false
            }
        }

        // 热门文章，取三篇
        pub fn hotPage(mut pc: PooledConn) -> Pages {
            let mut pages = Pages { pages: Vec::new() };
            let sql = format!("select * from Pages limit 3");

            pc.query_iter(sql).unwrap()
                .for_each(|row| {
                    let r: (String, String, String, String, String, String, String, String, String, String, String, String) = from_row(row.unwrap());
                    let page = Page {
                        title: r.0,
                        id: r.1,
                        author: r.2,
                        content: r.3,
                        date: r.4,
                        create_time: r.5,
                        cove_image: r.6,
                        summary: r.7,
                        read_number: r.8,
                        thumb_up_number: r.9,
                        comment_number: r.10,
                        tick: r.11
                    };
                    pages.pages.push(page)
                });

            pages
        }

        // 返回数据库中文章总数
        pub fn countPages(mut pc: PooledConn) -> i32 {
            let sql = format!("select count(*) from Pages");
            let res: Option<i32> = pc.query_first::<i32, String>(sql).unwrap();
            match res {
                Some(num) => return num,
                _ => return -1
            }
        }

        // 添加友链
        pub fn addFriend(mut pc: PooledConn, fl: FriendLink) -> bool {
            let sql = format!("insert into FriendLink(qq,name,url,descr) values('{}','{}','{}','{}')",
                fl.qq,
                fl.name,
                fl.url,
                fl.descr
            );

            let res: i32 = pc.query_first::<i32, String>(sql).unwrap().unwrap();

            if res > 0 {
                return true
            }
            false
        }

        // 返回友链列表
        pub fn friendLink(mut pc: PooledConn) -> FriendLinkS {
            let mut fls = FriendLinkS { friendLink: Vec::new() };
            let sql = "select * from FriendLink";

            pc.query_iter(sql).unwrap()
            .for_each(|row| {
                let r: (String, String, String, String) = from_row(row.unwrap());

                fls.friendLink.push(FriendLink {
                    qq: r.0,
                    name: r.1,
                    url: r.2,
                    descr: r.3
                })
            });

            fls
        }

        // 返回首页展示的信息
        pub fn websiteIndexMessageShow(mut pc: PooledConn) -> IndexPage {
            let sql = "select * from IndexPage";

            let mut iPag: IndexPage = IndexPage {
                name: String::new(),
                descr: String::new()
            };

            pc.query_iter(sql).unwrap()
                .for_each(|row| {
                    let r: (String, String) = from_row(row.unwrap());

                    iPag = IndexPage {
                        name: r.0,
                        descr: r.1
                    };
                });

            iPag
        }

        // 首页展示信息的修改
        pub fn websiteIndexMessageUpdate(mut pc: PooledConn, indexMessage: IndexPage) -> bool {
            let sql = format!("delete from IndexMessage set name={},descr={}",
                indexMessage.name,
                indexMessage.descr
            );

            let res: Option<i32>  = pc.query_first(sql).unwrap();
            match res {
                Some(num) => {
                    if num > 0 {
                        return true
                    }
                    return false
                },
                _ => return false
            }
        }
    }
}