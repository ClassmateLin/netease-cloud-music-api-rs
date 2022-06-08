//! # 云贝相关接口
use reqwest::Method;
use serde_json::{Value, json};
use crate::request::{request, CRYPTO_WEAPI, CRYPTO_EAPI, USER_AGENT_PC};

/// 获取历史签到信息
/// 
/// ### 示例:
/// ```
/// ncm_api::yunbei::get_total_sign_info().await?;
/// ```
pub async fn get_total_sign_info() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/point/signed/get";
    
    let data = json!({});

    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
        }
    });

    request(Method::POST, url, data, option).await
}


/// 获取今日签到信息
/// 
/// ### 示例:
/// ```
/// ncm_api::yunbei::get_today_sign_info().await?;
/// ```
pub async fn get_today_sign_info() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/point/today/get";
    
    let data = json!({});

    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
        }
    });

    request(Method::POST, url, data, option).await
}

/// 云贝签到
/// 
/// ### 示例:
/// ```
/// ncm_api::yunbei::sign().await?;
/// ```
pub async fn sign() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/point/dailyTask";
    
    let data = json!({
        "type": "0"
    });

    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
        }
    });

    request(Method::POST, url, data, option).await
}


/// 获取云贝信息
/// 
/// ### 示例:
/// ```
/// ncm_api::yunbei::get_info().await?;
/// ```
pub async fn get_info() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/v1/user/info";
    
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


/// 获取全部云贝任务
/// 
/// ### 示例:
/// ```
/// ncm_api::yunbei::get_all_tasks().await?;
/// ```
pub async fn get_all_tasks() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/usertool/task/list/all";
    
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


/// 获取待做云贝任务
/// 
/// ### 示例:
/// ```
/// ncm_api::yunbei::get_todo_tasks().await?;
/// ```
pub async fn get_todo_tasks() ->Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/api/usertool/task/todo/query";
    
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


/// 完成云贝任务
/// 
/// ### 参数:
/// - task_id: 可通过云贝任务列表接口获取。
/// ### 示例:
/// ```
/// ncm_api::yunbei::finish_task("12324").await?;
/// ```
pub async fn finish_task(task_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/weapi/usertool/task/point/receive";
    
    let data = json!({
        "userTaskId": task_id
    });

    let option = json!({
        "crypto": CRYPTO_WEAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
        }
    });

    request(Method::POST, url, data, option).await
}



/// 获取云贝收入记录
/// 
/// ## 参数:   
/// - limit: 取多少条数据
/// - offset: 跳过多少条数据
/// ### 示例:
/// ```
/// ncm_api::yunbei::get_receipt(100, 0).await?;
/// ```
pub async fn get_receipt(limit: i32, offset: i32) -> Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/store/api/point/receipt";
    
    let data = json!({
        "limit": limit, 
        "offset": offset
    });

    let option = json!({
        "crypto": CRYPTO_EAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
        }
    });

    request(Method::POST, url, data, option).await
}


/// 获取云贝支出记录
/// 
/// ## 参数:   
/// - limit: 取多少条数据
/// - offset: 跳过多少条数据
/// ### 示例:
/// ```
/// ncm_api::yunbei::get_expense(100, 0).await?;
/// ```
pub async fn get_expense(limit: i32, offset: i32) -> Result<Value, Box<dyn std::error::Error>> {
    let url = "https://music.163.com/store/api/point/expense";
    
    let data = json!({
        "limit": limit, 
        "offset": offset
    });

    let option = json!({
        "crypto": CRYPTO_EAPI,
        "ua": USER_AGENT_PC,
        "cookie": {
        }
    });
    request(Method::POST, url, data, option).await
}