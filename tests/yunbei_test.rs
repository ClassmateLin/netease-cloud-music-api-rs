#[tokio::test]
async fn test_get_total_sign_info(){
    let res = match ncm_api::yunbei::get_total_sign_info().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_get_today_sign_info(){
    let res = match ncm_api::yunbei::get_today_sign_info().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_sign(){
    let res = match ncm_api::yunbei::sign().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_get_info(){
    let res = match ncm_api::yunbei::get_info().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}


#[tokio::test]
async fn test_get_all_tasks(){
    let res = match ncm_api::yunbei::get_all_tasks().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_get_todo_tasks(){
    let res = match ncm_api::yunbei::get_todo_tasks().await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_finish_task(){
    let res = match ncm_api::yunbei::finish_task("1").await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}

#[tokio::test]
async fn test_get_receipt(){
    let res = match ncm_api::yunbei::get_receipt(10, 0).await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}


#[tokio::test]
async fn test_get_expense(){
    let res = match ncm_api::yunbei::get_expense(10, 0).await {
        Ok(_) => true,
        Err(_) => false,
    };
    assert_eq!(res, true);
}



