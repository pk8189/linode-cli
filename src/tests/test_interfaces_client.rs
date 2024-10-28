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
        .configs()
        .interfaces()
        .delete(linode_api::resources::linode::instances::configs::interfaces::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdApiVersionEnum::V4,
            linode_id: 123,
            config_id: 123,
            interface_id: 123,
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
        .configs()
        .interfaces()
        .list(linode_api::resources::linode::instances::configs::interfaces::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesApiVersionEnum::V4,
            linode_id: 123,
            config_id: 123,
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
        .configs()
        .interfaces()
        .get(linode_api::resources::linode::instances::configs::interfaces::GetRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdApiVersionEnum::V4,
            linode_id: 123,
            config_id: 123,
            interface_id: 123,
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
        .configs()
        .interfaces()
        .create(linode_api::resources::linode::instances::configs::interfaces::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesApiVersionEnum::V4,
            linode_id: 123,
            config_id: 123,
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
        .linode()
        .instances()
        .configs()
        .interfaces()
        .put(linode_api::resources::linode::instances::configs::interfaces::PutRequest {
            api_version: linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdApiVersionEnum::V4,
            linode_id: 123,
            config_id: 123,
            interface_id: 123,
            data: linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdBody {
                ip_ranges: linode_api::Patch::new(
                    vec![
                        "10.0.0.64/26".to_string(), "fd04:495a:691c:971c::1:0/112"
                        .to_string()
                    ],
                ),
                ipv4: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdBodyIpv4 {
                    nat_1_1: linode_api::Patch::new("203.0.113.2".to_string()),
                    vpc: linode_api::Patch::new("10.0.0.2".to_string()),
                    ..Default::default()
                }),
                primary: Some(true),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
