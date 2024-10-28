#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .promo_codes()
        .create(linode_api::resources::account::promo_codes::CreateRequest {
            api_version: linode_api::models::PostApiVersionAccountPromoCodesApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionAccountPromoCodesBody {
                promo_code: "string".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
