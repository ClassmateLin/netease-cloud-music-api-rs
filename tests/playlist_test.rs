#[tokio::test]
async fn test_get_user_playlist(){
    let res = match ncm_api::playlist::get_user_playlist("32423432", 10, 0).await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}