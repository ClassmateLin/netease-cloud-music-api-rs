//! # 歌单相关接口

use reqwest::Method;
use serde_json::{Value, json};

use crate::request::{request, CRYPTO_WEAPI, USER_AGENT_PC};


/// 获取某用户歌单
/// ## 参数:   
/// - uid: 用户ID
/// - limit: 取多少条数据
/// - offset: 跳过多少条数据
/// ### 示例:
/// ```
/// ncm_api::playlist::get_user_playlist("1", 100, 0).await?;
/// ```
pub async fn get_user_playlist(uid: &str, limit: i32, offset: i32) ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/user/playlist";
    
    let data = json!(
        {
            "uid": uid,
            "limit": limit,
            "offset": offset,
        }
    );

    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
        }
    });

    request(Method::POST, url, data, option).await
}
