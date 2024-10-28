#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .payment_methods()
        .make_default()
        .create(linode_api::resources::account::payment_methods::make_default::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountPaymentMethodsPaymentMethodIdMakeDefaultApiVersionEnum::V4,
            payment_method_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
