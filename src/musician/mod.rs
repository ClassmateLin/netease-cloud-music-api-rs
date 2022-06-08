 //! # 音乐人相关接口

use reqwest::Method;
use serde_json::{Value, json};

use crate::request::{request, CRYPTO_WEAPI, USER_AGENT_MOBILE};

/// 获取音乐人任务列表
/// ### 示例:
/// ```
/// ncm_api::musician::get_tasks().await?;
/// ```
pub async fn get_tasks() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/nmusician/workbench/mission/stage/list";
    let data = json!({});
    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_MOBILE,
        "cookie": {
        }
    });
    request(Method::POST, url, data, option).await
}

/// 获取云豆数量信息
/// ### 示例:
/// ```
/// ncm_api::musician::get_cloudbean_info().await?;
/// ```
pub async fn get_cloudbean_info() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/cloudbean/get";
    let data = json!({});
    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_MOBILE,
        "cookie": {
        }
    });
    request(Method::POST, url, data, option).await
}

/// 获取云豆
/// ## 参数:   
/// - mission_id: 任务ID, 可通过ncm_api::musician::get_tasks()获取。
/// - period: 可通过ncm_api::musician::get_tasks()获取。
/// ### 示例:
/// ```
/// ncm_api::musician::get_cloudbean_info("1", "zxx").await?;
/// ```
pub async fn obtain_cloudbean(mission_id: &str, period: &str)->Result<Value, Box<dyn std::error::Error>>{
    let url = "https://music.163.com/weapi/nmusician/workbench/mission/reward/obtain/new";
    let data = json!({
        "period": period,
        "userMissionId": mission_id,
    });
    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_MOBILE,
        "cookie": {
        }
    });
    request(Method::POST, url, data, option).await
}