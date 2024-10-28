#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .volumes()
        .clone()
        .create(linode_api::resources::volumes::clone::CreateRequest {
            api_version: linode_api::models::PostApiVersionVolumesVolumeIdCloneApiVersionEnum::V4,
            volume_id: 123,
            data: linode_api::models::PostApiVersionVolumesVolumeIdCloneBody {
                label: "my-volume".to_string(),
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
        .clone()
        .create(linode_api::resources::linode::instances::clone::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdCloneApiVersionEnum::V4,
            linode_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdCloneBody {
                backups_enabled: Some(true),
                configs: Some(vec![23456]),
                disks: Some(vec![25674]),
                group: Some("Linode-Group".to_string()),
                label: Some("cloned-linode".to_string()),
                linode_id: Some(124),
                metadata: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdCloneBodyMetadata {
                    user_data: Some(
                        "I2Nsb3VkLWNvbmZpZwpwYWNrYWdlX3VwZGF0ZTogdHJ1ZQpwYWNrYWdlX3VwZ3JhZGU6IHRydWU="
                            .to_string(),
                    ),
                    ..Default::default()
                }),
                placement_group: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdCloneBodyPlacementGroup {
                    id: 528,
                    ..Default::default()
                }),
                private_ip: Some(true),
                region: Some("us-east".to_string()),
                type_field: Some("g6-standard-2".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .linode()
        .instances()
        .disks()
        .clone()
        .create(linode_api::resources::linode::instances::disks::clone::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdCloneApiVersionEnum::V4,
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
        .domains()
        .clone()
        .create(linode_api::resources::domains::clone::CreateRequest {
            api_version: linode_api::models::PostApiVersionDomainsDomainIdCloneApiVersionEnum::V4,
            domain_id: "string".to_string(),
            data: linode_api::models::PostApiVersionDomainsDomainIdCloneBody {
                domain: "example.org".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
