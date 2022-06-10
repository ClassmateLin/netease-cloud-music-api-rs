#[tokio::test]
async fn test_login_by_email() {
    let res = match ncm_api::login::login_by_email("xxx", "xx").await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_login_by_token() {
    let res = match ncm_api::login::login_by_token("xxx", "xx").await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_refresh_login_status() {
    let res = match ncm_api::login::refresh_login_status().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_get_login_status(){
    let res = match ncm_api::login::get_login_status().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_logout(){
    let res = match ncm_api::login::logout().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}