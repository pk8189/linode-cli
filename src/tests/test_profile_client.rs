#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .profile_resource()
        .list(linode_api::resources::profile_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionProfileApiVersionEnum::V4,
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
        .profile_resource()
        .put(linode_api::resources::profile_resource::PutRequest {
            api_version: linode_api::models::PutApiVersionProfileApiVersionEnum::V4,
            data: linode_api::models::PutApiVersionProfileBody {
                authentication_type: Some(
                    linode_api::models::PutApiVersionProfileBodyAuthenticationTypeEnum::Password,
                ),
                authorized_keys: linode_api::Patch::new(vec!["string".to_string()]),
                email: Some("example-user@gmail.com".to_string()),
                email_notifications: Some(true),
                ip_whitelist_enabled: Some(false),
                lish_auth_method: Some(
                    linode_api::models::PutApiVersionProfileBodyLishAuthMethodEnum::KeysOnly,
                ),
                referrals: Some(linode_api::models::PutApiVersionProfileBodyReferrals {
                    code: Some("871be32f49c1411b14f29f618aaf0c14637fb8d3".to_string()),
                    completed: Some(0),
                    credit: Some(0),
                    pending: Some(0),
                    total: Some(0),
                    url: Some(
                        "https://www.linode.com/?r=871be32f49c1411b14f29f618aaf0c14637fb8d3"
                            .to_string(),
                    ),
                    ..Default::default()
                }),
                restricted: Some(false),
                timezone: Some("US/Eastern".to_string()),
                two_factor_auth: Some(true),
                uid: Some(1234),
                username: Some("exampleUser".to_string()),
                verified_phone_number: linode_api::Patch::new("+5555555555".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
