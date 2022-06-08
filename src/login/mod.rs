 //! # 登录相关接口

use openssl::hash::{MessageDigest, hash};
use reqwest::Method;
use serde_json::{Value, json};
use crate::request::{request, CRYPTO_WEAPI, USER_AGENT_PC};

/// 网易邮箱登录
/// 
/// 登录成功后, 请求其他API会自动带上登录状态。
/// # 参数:
/// - email: 网易邮箱账号
/// - password:  明文密码
/// ### 示例:
/// ```
/// ncm_api::login::login_by_email("test", "123456").await?;
/// ```
pub async fn login_by_email(email: &str, password:&str) -> Result<Value, Box<dyn std::error::Error>> {
    
    let url = "https://music.163.com/weapi/login";

    let password_md5 = hex::encode(hash(MessageDigest::md5(), password.as_bytes()).unwrap());

    let data = json!({
        "username": email,
        "password": password_md5,
        "rememberLogin": true,
    });

    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
            "appver": "2.9.7",
            "os": USER_AGENT_PC
        }
    });
    
    request(Method::POST, url, data, option).await
}

/// 查询登录状态
/// ### 示例:
/// ```
/// ncm_api::login::get_login_status().await?;
/// ```
pub async fn get_login_status() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/w/nuser/account/get";

    let data = json!({});

    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
        }
    });

    request(Method::POST, url, data, option).await
}

/// 刷新登录状态
/// ### 示例:
/// ```
/// ncm_api::login::refresh_login_status().await?;
/// ```
pub async fn refresh_login_status() ->Result<Value, Box<dyn std::error::Error>> {
    
    let url = "https://music.163.com/weapi/login/token/refresh";
    
    let data = json!({});

    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
            "appver": "2.9.7",
            "os": USER_AGENT_PC
        }
    });

    request(Method::POST, url, data, option).await
}


/// 注销登录
/// ### 示例:
/// ```
/// ncm_api::login::logout().await?;
/// ```
pub async fn logout() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/logout";

    let data = json!({});

    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
            "appver": "2.9.7",
            "os": USER_AGENT_PC
        }
    });

    request(Method::POST, url, data, option).await
}

