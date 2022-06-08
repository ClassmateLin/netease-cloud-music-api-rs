use rand::prelude::SliceRandom;
use regex::Regex;
use serde_json::{Value, Map};
use reqwest::Method;
use crate::crypto::Crypto;
const CACHE_PATH:&str = "./.ncm_cache";
pub const USER_AGENT_PC:&str =  "PC";
pub const USER_AGENT_MOBILE: &str = "Mobile";
pub const CACHE_CSRF_TOKEN_KEY: &str = "LOGIN_CSRF_TOKEN";
pub const CACHE_TOKEN_KEY: &str = "LOGIN_TOKEN";
pub const CRYPTO_WEAPI:&str = "WEAPI";
pub const CRYPTO_LINUX_API: &str = "LINUXAPI";
pub const CRYPTO_EAPI:&str = "EAPI";

lazy_static! {
    static ref PC_USER_AGENT_LIST:Vec<&'static str> = vec![
      // macOS 10.15.6  Firefox / Chrome / Safari
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:80.0) Gecko/20100101 Firefox/80.0",
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.30 Safari/537.36",
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.2 Safari/605.1.15",
      // Windows 10 Firefox / Chrome / Edge
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:80.0) Gecko/20100101 Firefox/80.0",
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/86.0.4240.30 Safari/537.36",
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/13.10586",
    ];

    static ref MOBILE_USER_AGENT_LIST:Vec<&'static str> = vec![
              // iOS 13.5.1 14.0 beta with safari
      "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.1 Mobile/15E148 Safari/604.1",
      "Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.",
      // iOS with qq micromsg
      "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/602.1.50 (KHTML like Gecko) Mobile/14A456 QQ/6.5.7.408 V1_IPH_SQ_6.5.7_1_APP_A Pixel/750 Core/UIWebView NetType/4G Mem/103",
      "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148 MicroMessenger/7.0.15(0x17000f27) NetType/WIFI Language/zh",
      // Android -> Huawei Xiaomi
      "Mozilla/5.0 (Linux; Android 9; PCT-AL10) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.64 HuaweiBrowser/10.0.3.311 Mobile Safari/537.36",
      "Mozilla/5.0 (Linux; U; Android 9; zh-cn; Redmi Note 8 Build/PKQ1.190616.001) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/71.0.3578.141 Mobile Safari/537.36 XiaoMi/MiuiBrowser/12.5.22",
      // Android + qq micromsg
      "Mozilla/5.0 (Linux; Android 10; YAL-AL00 Build/HUAWEIYAL-AL00; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/78.0.3904.62 XWEB/2581 MMWEBSDK/200801 Mobile Safari/537.36 MMWEBID/3027 MicroMessenger/7.0.18.1740(0x27001235) Process/toolsmp WeChat/arm64 NetType/WIFI Language/zh_CN ABI/arm64",
      "Mozilla/5.0 (Linux; U; Android 8.1.0; zh-cn; BKK-AL10 Build/HONORBKK-AL10) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/66.0.3359.126 MQQBrowser/10.6 Mobile Safari/537.36",
    ];

    static ref LINUX_USER_AGENT_LIST:Vec<&'static str> = vec![
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:90.0) Gecko/20100101 Firefox/90.0",
        "Mozilla/5.0 (X11; Linux x86_64; rv:90.0) Gecko/20100101 Firefox/90.0"
    ];

    static ref CSRF_REGEX: Regex = Regex::new(r"__csrf=(?P<csrf>[^(;|$)]+)").unwrap();

    static ref TOKEN_REGEX: Regex = Regex::new(r"MUSIC_U=(?P<token>[^(;|$)]+)").unwrap();
}

pub async fn request(method: Method, url: &str, data: Value, option: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let user_agent = match option.get("ua").unwrap().to_string().as_str() {
        USER_AGENT_PC => PC_USER_AGENT_LIST.choose(&mut rand::thread_rng()).unwrap().to_string(),
        USER_AGENT_MOBILE => MOBILE_USER_AGENT_LIST.choose(&mut rand::thread_rng()).unwrap().to_string(),
        _ => LINUX_USER_AGENT_LIST.choose(&mut rand::thread_rng()).unwrap().to_string(),
    };
    
    let cookie_map:Map<String, Value>= serde_json::from_value(option.get("cookie").unwrap().clone()).unwrap();
    let mut cookie_str: String = String::new();
    for item in cookie_map {
        cookie_str += &format!("{}={};", item.0, item.1);
    }
    let csrf_token = String::from_utf8(cacache::read(CACHE_PATH, CACHE_CSRF_TOKEN_KEY).await.unwrap_or_default()).unwrap_or_default();
    let token = String::from_utf8(cacache::read(CACHE_PATH, CACHE_TOKEN_KEY).await.unwrap_or_default()).unwrap_or_default();

    cookie_str += &format!("MUSIC_U={:?};", token);
    cookie_str += &format!("__csrf={:?};", csrf_token);
    cookie_str = cookie_str.replace('"', "");
    
    let crypto = option.get("crypto").unwrap().as_str().unwrap();
    let body = match crypto {
        CRYPTO_WEAPI => {Crypto::weapi(data.to_string().as_str())},
        CRYPTO_LINUX_API => Crypto::linuxapi(data.to_string().as_str()),
        _ => Crypto::eapi(url, data.to_string().as_str()),
    };

    let response = client.request(method, url)
        .header("user-agent", user_agent)
        .header("content-type", "application/x-www-form-urlencoded")
        .header("cookie", cookie_str)
        .header("Referer", "https://music.163.com")
        .body(body)
        .send()
        .await?;
        
    if csrf_token.is_empty(){
        for (k, v) in response.headers() {
            let v = v.to_str().unwrap_or("");
            if k.eq("set-cookie") && v.contains("_csrf") {
                let csrf = if let Some(caps) = CSRF_REGEX.captures(v) {
                    caps.name("csrf").unwrap().as_str().to_string()
                } else {
                    "".to_string()
                };
                cacache::write(CACHE_PATH, CACHE_CSRF_TOKEN_KEY, csrf).await?;
            }
            
            if k.eq("set-cookie") && v.contains("MUSIC_U") {
                let token = if let Some(caps) = TOKEN_REGEX.captures(v) {
                    caps.name("token").unwrap().as_str().to_string()
                } else {
                    "".to_string()
                };
                cacache::write(CACHE_PATH, CACHE_TOKEN_KEY, token).await?;
            }
            
            
        }
    }


    Ok(Value::from(response.text().await?))

}