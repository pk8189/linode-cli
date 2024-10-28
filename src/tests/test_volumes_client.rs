#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .volumes()
        .delete(linode_api::resources::volumes::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionVolumesVolumeIdApiVersionEnum::V4,
            volume_id: 123,
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
        .volumes()
        .list(linode_api::resources::volumes::ListRequest {
            api_version: linode_api::models::GetApiVersionVolumesApiVersionEnum::V4,
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
        .volumes()
        .get(linode_api::resources::volumes::GetRequest {
            api_version: linode_api::models::GetApiVersionVolumesVolumeIdApiVersionEnum::V4,
            volume_id: 123,
            ..Default::default()
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
        .volumes()
        .create(linode_api::resources::volumes::CreateRequest {
            api_version: linode_api::models::PostApiVersionVolumesApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionVolumesBody {
                config_id: Some(23456),
                encryption: Some(
                    linode_api::models::PostApiVersionVolumesBodyEncryptionEnum::Enabled,
                ),
                label: "my-volume".to_string(),
                linode_id: Some(123),
                region: Some("string".to_string()),
                size: Some(20),
                tags: Some(
                    vec!["example tag".to_string(), "another example".to_string()],
                ),
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
        .volumes()
        .put(linode_api::resources::volumes::PutRequest {
            api_version: linode_api::models::PutApiVersionVolumesVolumeIdApiVersionEnum::V4,
            volume_id: 123,
            data: "could be anything",
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
        .linode()
        .instances()
        .volumes()
        .list(linode_api::resources::linode::instances::volumes::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdVolumesApiVersionEnum::V4,
            linode_id: 123,
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
