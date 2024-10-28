#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .payments()
        .paypal()
        .execute()
        .create(linode_api::resources::account::payments::paypal::execute::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountPaymentsPaypalExecuteApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountPaymentsPaypalExecuteBody {
                payer_id: "ABCDEFGHIJKLM".to_string(),
                payment_id: "PAY-1234567890ABCDEFGHIJKLMN".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_202_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .payments()
        .paypal()
        .execute()
        .create(linode_api::resources::account::payments::paypal::execute::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountPaymentsPaypalExecuteApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountPaymentsPaypalExecuteBody {
                payer_id: "ABCDEFGHIJKLM".to_string(),
                payment_id: "PAY-1234567890ABCDEFGHIJKLMN".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_299_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .payments()
        .paypal()
        .execute()
        .create(linode_api::resources::account::payments::paypal::execute::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountPaymentsPaypalExecuteApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountPaymentsPaypalExecuteBody {
                payer_id: "ABCDEFGHIJKLM".to_string(),
                payment_id: "PAY-1234567890ABCDEFGHIJKLMN".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
