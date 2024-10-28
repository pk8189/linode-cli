#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .payment_methods()
        .delete(linode_api::resources::account::payment_methods::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionAccountPaymentMethodsPaymentMethodIdApiVersionEnum::V4,
            payment_method_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .payment_methods()
        .list(linode_api::resources::account::payment_methods::ListRequest {
            api_version: linode_api::models::GetApiVersionAccountPaymentMethodsApiVersionEnum::V4,
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_get_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .payment_methods()
        .get(linode_api::resources::account::payment_methods::GetRequest {
            api_version: linode_api::models::GetApiVersionAccountPaymentMethodsPaymentMethodIdApiVersionEnum::V4,
            payment_method_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .payment_methods()
        .create(linode_api::resources::account::payment_methods::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountPaymentMethodsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountPaymentMethodsBody {
                data: linode_api::models::PostApiVersionAccountPaymentMethodsBodyData {
                    card_number: "string".to_string(),
                    cvv: "123".to_string(),
                    expiry_month: 12,
                    expiry_year: 2020,
                    ..Default::default()
                },
                is_default: true,
                type_field: linode_api::models::PostApiVersionAccountPaymentMethodsBodyTypeEnum::CreditCard,
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
