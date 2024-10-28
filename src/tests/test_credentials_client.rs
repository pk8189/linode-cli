#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .managed()
        .credentials()
        .list(linode_api::resources::managed::credentials::ListRequest {
            api_version: linode_api::models::GetApiVersionManagedCredentialsApiVersionEnum::V4,
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
        .managed()
        .credentials()
        .get(linode_api::resources::managed::credentials::GetRequest {
            api_version: linode_api::models::GetApiVersionManagedCredentialsCredentialIdApiVersionEnum::V4,
            credential_id: 123,
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
        .managed()
        .credentials()
        .create(linode_api::resources::managed::credentials::CreateRequest {
            api_version: linode_api::models::PostApiVersionManagedCredentialsApiVersionEnum::V4,
            data: "could be anything",
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
        .managed()
        .credentials()
        .put(linode_api::resources::managed::credentials::PutRequest {
            api_version: linode_api::models::PutApiVersionManagedCredentialsCredentialIdApiVersionEnum::V4,
            credential_id: 123,
            data: linode_api::models::PutApiVersionManagedCredentialsCredentialIdBody {
                id: Some(9991),
                label: Some("prod-password-1".to_string()),
                last_decrypted: Some("2018-01-01T00:01:01".to_string()),
                ..Default::default()
            },
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
        .databases()
        .postgresql()
        .instances()
        .credentials()
        .list(linode_api::resources::databases::postgresql::instances::credentials::ListRequest {
            api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdCredentialsApiVersionEnum::V4,
            instance_id: 123,
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
        .databases()
        .mysql()
        .instances()
        .credentials()
        .list(linode_api::resources::databases::mysql::instances::credentials::ListRequest {
            api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesInstanceIdCredentialsApiVersionEnum::V4,
            instance_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
