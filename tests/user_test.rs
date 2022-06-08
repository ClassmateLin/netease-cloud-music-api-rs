#[tokio::test]
async fn test_daily_sign(){
    let res = match ncm_api::user::daily_sign().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}


#[tokio::test]
async fn get_account_info(){
    let res = match ncm_api::user::get_account_info().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}