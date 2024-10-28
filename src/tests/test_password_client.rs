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
        .password()
        .create(linode_api::resources::linode::instances::password::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdPasswordApiVersionEnum::V4,
            linode_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdPasswordBody {
                root_pass: "a$eCure4assw0rd!43v51".to_string(),
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
        .password()
        .create(linode_api::resources::linode::instances::disks::password::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdPasswordApiVersionEnum::V4,
            linode_id: 123,
            disk_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdDisksDiskIdPasswordBody {
                password: "another@complex^Password123".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
