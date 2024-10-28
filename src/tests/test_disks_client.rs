#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .linode()
        .instances()
        .disks()
        .delete(linode_api::resources::linode::instances::disks::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLinodeInstancesLinodeIdDisksDiskIdApiVersionEnum::V4,
            linode_id: 123,
            disk_id: 123,
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
        .disks()
        .list(linode_api::resources::linode::instances::disks::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdDisksApiVersionEnum::V4,
            linode_id: 123,
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
        .linode()
        .instances()
        .disks()
        .get(linode_api::resources::linode::instances::disks::GetRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdDisksDiskIdApiVersionEnum::V4,
            linode_id: 123,
            disk_id: 123,
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
        .linode()
        .instances()
        .disks()
        .create(linode_api::resources::linode::instances::disks::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksApiVersionEnum::V4,
            linode_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksBody {
                authorized_keys: Some(
                    vec![
                        "ssh-rsa AAAA_valid_public_ssh_key_123456785== user@their-computer"
                        .to_string()
                    ],
                ),
                authorized_users: Some(
                    vec!["myUser".to_string(), "secondaryUser".to_string()],
                ),
                filesystem: Some(
                    linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksBodyFilesystemEnum::Ext4,
                ),
                image: Some("linode/debian9".to_string()),
                label: Some("Debian 9 Disk".to_string()),
                root_pass: Some("aComplexP@ssword".to_string()),
                size: Some(48640),
                stackscript_data: Some(serde_json::json!({ "gh_username" : "linode" })),
                stackscript_id: Some(10079),
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
        .linode()
        .instances()
        .disks()
        .put(linode_api::resources::linode::instances::disks::PutRequest {
            api_version: linode_api::models::PutApiVersionLinodeInstancesLinodeIdDisksDiskIdApiVersionEnum::V4,
            linode_id: 123,
            disk_id: 123,
            data: linode_api::models::PutApiVersionLinodeInstancesLinodeIdDisksDiskIdBody {
                label: Some("Debian 9 Disk".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
