#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .profile_resource()
        .devices()
        .delete(linode_api::resources::profile_resource::devices::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionProfileDevicesDeviceIdApiVersionEnum::V4,
            device_id: 123,
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
        .profile_resource()
        .devices()
        .list(linode_api::resources::profile_resource::devices::ListRequest {
            api_version: linode_api::models::GetApiVersionProfileDevicesApiVersionEnum::V4,
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
        .profile_resource()
        .devices()
        .get(linode_api::resources::profile_resource::devices::GetRequest {
            api_version: linode_api::models::GetApiVersionProfileDevicesDeviceIdApiVersionEnum::V4,
            device_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .networking()
        .firewalls()
        .devices()
        .delete(linode_api::resources::networking::firewalls::devices::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdApiVersionEnum::V4,
            firewall_id: 123,
            device_id: 123,
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
        .networking()
        .firewalls()
        .devices()
        .list(linode_api::resources::networking::firewalls::devices::ListRequest {
            api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdDevicesApiVersionEnum::V4,
            firewall_id: 123,
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
        .networking()
        .firewalls()
        .devices()
        .get(linode_api::resources::networking::firewalls::devices::GetRequest {
            api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdApiVersionEnum::V4,
            firewall_id: 123,
            device_id: 123,
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
        .networking()
        .firewalls()
        .devices()
        .create(linode_api::resources::networking::firewalls::devices::CreateRequest {
            api_version: linode_api::models::PostApiVersionNetworkingFirewallsFirewallIdDevicesApiVersionEnum::V4,
            firewall_id: 123,
            data: linode_api::models::PostApiVersionNetworkingFirewallsFirewallIdDevicesBody {
                id: Some(123),
                label: Some("my-linode".to_string()),
                type_field: Some(
                    linode_api::models::PostApiVersionNetworkingFirewallsFirewallIdDevicesBodyTypeEnum::Linode,
                ),
                url: Some("/v4/linode/instances/123".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
