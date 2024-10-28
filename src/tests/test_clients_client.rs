#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .longview()
        .clients()
        .delete(linode_api::resources::longview::clients::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLongviewClientsClientIdApiVersionEnum::V4,
            client_id: 123,
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
        .longview()
        .clients()
        .list(linode_api::resources::longview::clients::ListRequest {
            api_version: linode_api::models::GetApiVersionLongviewClientsApiVersionEnum::V4,
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
        .longview()
        .clients()
        .get(linode_api::resources::longview::clients::GetRequest {
            api_version: linode_api::models::GetApiVersionLongviewClientsClientIdApiVersionEnum::V4,
            client_id: 123,
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
        .longview()
        .clients()
        .create(linode_api::resources::longview::clients::CreateRequest {
            api_version: linode_api::models::PostApiVersionLongviewClientsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionLongviewClientsBody {
                api_key: Some("BD1B4B54-D752-A76D-5A9BD8A17F39DB61".to_string()),
                apps: Some(linode_api::models::PostApiVersionLongviewClientsBodyApps {
                    apache: Some(true),
                    mysql: Some(true),
                    nginx: Some(false),
                    ..Default::default()
                }),
                created: Some("2018-01-01T00:01:01".to_string()),
                id: Some(789),
                install_code: Some("BD1B5605-BF5E-D385-BA07AD518BE7F321".to_string()),
                label: Some("client789".to_string()),
                updated: Some("2018-01-01T00:01:01".to_string()),
                ..Default::default()
            },
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
        .longview()
        .clients()
        .put(linode_api::resources::longview::clients::PutRequest {
            api_version: linode_api::models::PutApiVersionLongviewClientsClientIdApiVersionEnum::V4,
            client_id: 123,
            data: linode_api::models::PutApiVersionLongviewClientsClientIdBody {
                api_key: Some("BD1B4B54-D752-A76D-5A9BD8A17F39DB61".to_string()),
                apps: Some(linode_api::models::PutApiVersionLongviewClientsClientIdBodyApps {
                    apache: Some(true),
                    mysql: Some(true),
                    nginx: Some(false),
                    ..Default::default()
                }),
                created: Some("2018-01-01T00:01:01".to_string()),
                id: Some(789),
                install_code: Some("BD1B5605-BF5E-D385-BA07AD518BE7F321".to_string()),
                label: Some("client789".to_string()),
                updated: Some("2018-01-01T00:01:01".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
