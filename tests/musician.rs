#[tokio::test]
async fn test_get_tasks(){
    let res = match ncm_api::musician::get_tasks().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}


#[tokio::test]
async fn test_get_cloudbean_info(){
    let res = match ncm_api::musician::get_cloudbean_info().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_obtain_cloudbean(){
    let res = match ncm_api::musician::obtain_cloudbean("1", "1").await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

