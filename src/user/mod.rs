//! # 用户相关接口
use reqwest::Method;
use serde_json::{Value, json};

use crate::request::{request, CRYPTO_WEAPI, USER_AGENT_PC};


/// 每日签到
/// 
/// 签到奖励经验值, 使用的是安卓签到, 获得的经验值最多。
/// ### 示例:
/// ```
/// ncm_api::user::daily_sign().await?;
/// ```
pub async fn daily_sign() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/point/dailyTask";
    let data = json!({
        "type": "0",
    });
    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
        }
    });
    request(Method::POST, url, data, option).await
}

/// 获取当前登录的账号信息
/// 
/// ### 示例:
/// ```
/// ncm_api::user::get_account_info().await?;
/// ```
pub async fn get_account_info() ->Result<Value, Box<dyn std::error::Error>> {
    let url = &format!("https://music.163.com/weapi/nuser/account/get");

    let data = json!({
    });

    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
        }
    });
    request(Method::POST, url, data, option).await
}


