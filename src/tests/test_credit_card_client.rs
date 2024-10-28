#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .credit_card()
        .create(linode_api::resources::account::credit_card::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountCreditCardApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountCreditCardBody {
                card_number: "string".to_string(),
                cvv: "123".to_string(),
                expiry_month: 12,
                expiry_year: 2020,
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
