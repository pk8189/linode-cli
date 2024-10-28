#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .volumes()
        .resize()
        .create(linode_api::resources::volumes::resize::CreateRequest {
            api_version: linode_api::models::PostApiVersionVolumesVolumeIdResizeApiVersionEnum::V4,
            volume_id: 123,
            data: linode_api::models::PostApiVersionVolumesVolumeIdResizeBody {
                size: 30,
                ..Default::default()
            },
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
        .resize()
        .create(linode_api::resources::linode::instances::resize::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdResizeApiVersionEnum::V4,
            linode_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdResizeBody {
                allow_auto_disk_resize: Some(true),
                migration_type: Some(
                    linode_api::models::PostApiVersionLinodeInstancesLinodeIdResizeBodyMigrationTypeEnum::Warm,
                ),
                type_field: "g6-standard-2".to_string(),
                ..Default::default()
            },
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
        .resize()
        .create(linode_api::resources::linode::instances::disks::resize::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdResizeApiVersionEnum::V4,
            linode_id: 123,
            disk_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdResizeBody {
                size: 2048,
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
