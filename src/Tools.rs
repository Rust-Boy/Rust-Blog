#[allow(warnings)]
pub mod Jwt {
    use std::fs;
    use crypto::digest::Digest;
    use crypto::md5::Md5;
    use crate::Model::model::Admin;

    pub fn md5<S:Into<String>>(input: S) -> String {
        let mut md5 = Md5::new();
        md5.input_str(&input.into());
        md5.result_str()
    }

    // 生成一个 jwt
    pub fn jwt(admin: Admin) -> String {
        let flag = "jwt";
        let jwt = md5(format!("{}-{}-{}", flag, admin.username, admin.password));
        let jwt = format!("{} 2022-1-14", jwt);
        let jwt = md5(jwt);
        jwt
    }

    // 签发 jwt
    pub fn jwtCreate(admin: Admin) -> String {
        let jwt = jwt(admin);

        let path = "src/config/jwt.txt";
        fs::write(path, jwt.clone());
        jwt
    }

    // 通过判断 Jwt 是否存在来判断是否登录
    pub fn isLogin(jwt: String) -> bool {
        let path = "src/config/jwt.txt";

        let _jwt = fs::read_to_string(path).unwrap();
        let _jwt = _jwt.trim();

        if _jwt == jwt {
            return true
        }

        false
    }

    // 删除 jwt
    pub fn jwtDel() {
        let path = "src/config/jwt.txt";
        fs::write(path, "");
    }
}