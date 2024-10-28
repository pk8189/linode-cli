#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .list(linode_api::resources::account::ListRequest {
            api_version: linode_api::models::GetApiVersionAccountApiVersionEnum::V4,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_put_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .put(linode_api::resources::account::PutRequest {
            api_version: linode_api::models::PutApiVersionAccountApiVersionEnum::V4,
            data: linode_api::models::PutApiVersionAccountBody {
                active_promotions: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountBodyActivePromotionsItem
                        { credit_monthly_cap : Some("10.00".to_string()),
                        credit_remaining : Some("50.00".to_string()), description :
                        Some("Receive up to $10 off your services every month for 6 months! Unused credits will expire once this promotion period ends."
                        .to_string()), expire_dt : Some("2018-01-31T23:59:59"
                        .to_string()), image_url :
                        Some("https://linode.com/10_a_month_promotion.svg".to_string()),
                        service_type :
                        Some(linode_api::models::PutApiVersionAccountBodyActivePromotionsItemServiceTypeEnum::All),
                        summary : Some("$10 off your Linode a month!".to_string()),
                        this_month_credit_remaining : Some("10.00".to_string()),
                        ..Default::default() }
                    ],
                ),
                active_since: Some("2018-01-01T00:01:01".to_string()),
                address_1: Some("123 Main Street".to_string()),
                address_2: Some("Suite A".to_string()),
                balance: Some(200),
                balance_uninvoiced: Some(145),
                billing_source: Some(
                    linode_api::models::PutApiVersionAccountBodyBillingSourceEnum::Akamai,
                ),
                capabilities: Some(
                    vec![
                        "Linodes".to_string(), "NodeBalancers".to_string(),
                        "Block Storage".to_string(), "Object Storage".to_string(),
                        "Placement Groups".to_string(), "Block Storage Encryption"
                        .to_string()
                    ],
                ),
                city: Some("Philadelphia".to_string()),
                company: Some("Linode LLC".to_string()),
                country: Some("US".to_string()),
                credit_card: Some(linode_api::models::PutApiVersionAccountBodyCreditCard {
                    expiry: Some("11/2022".to_string()),
                    last_four: Some("string".to_string()),
                    ..Default::default()
                }),
                email: Some("john.smith@linode.com".to_string()),
                euuid: Some("E1AF5EEC-526F-487D-B317EBEB34C87D71".to_string()),
                first_name: Some("John".to_string()),
                last_name: Some("Smith".to_string()),
                phone: Some("215-555-1212".to_string()),
                state: Some("PA".to_string()),
                tax_id: Some("ATU99999999".to_string()),
                zip: Some("19102-1234".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
