#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .vpcs()
        .ips()
        .list(linode_api::resources::vpcs::ips::ListRequest {
            api_version: linode_api::models::GetApiVersionVpcsIpsApiVersionEnum::V4,
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_1_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .vpcs()
        .ips()
        .list_1(linode_api::resources::vpcs::ips::List1Request {
            api_version: linode_api::models::GetApiVersionVpcsVpcIdIpsApiVersionEnum::V4,
            vpc_id: 123,
            ..Default::default()
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
        .ips()
        .list(linode_api::resources::networking::ips::ListRequest {
            api_version: linode_api::models::GetApiVersionNetworkingIpsApiVersionEnum::V4,
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
        .ips()
        .get(linode_api::resources::networking::ips::GetRequest {
            api_version: linode_api::models::GetApiVersionNetworkingIpsAddressApiVersionEnum::V4,
            address: "string".to_string(),
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
        .ips()
        .create(linode_api::resources::networking::ips::CreateRequest {
            api_version: linode_api::models::PostApiVersionNetworkingIpsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionNetworkingIpsBody {
                linode_id: 123,
                public: true,
                type_field: linode_api::models::PostApiVersionNetworkingIpsBodyTypeEnum::Ipv4,
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
        .networking()
        .ips()
        .put(linode_api::resources::networking::ips::PutRequest {
            api_version: linode_api::models::PutApiVersionNetworkingIpsAddressApiVersionEnum::V4,
            address: "string".to_string(),
            data: linode_api::models::PutApiVersionNetworkingIpsAddressBody {
                rdns: Some("test.example.org".to_string()),
                ..Default::default()
            },
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
        .linode()
        .instances()
        .ips()
        .delete(linode_api::resources::linode::instances::ips::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLinodeInstancesLinodeIdIpsAddressApiVersionEnum::V4,
            linode_id: 123,
            address: "string".to_string(),
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
        .ips()
        .list(linode_api::resources::linode::instances::ips::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdIpsApiVersionEnum::V4,
            linode_id: 123,
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
        .ips()
        .get(linode_api::resources::linode::instances::ips::GetRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdIpsAddressApiVersionEnum::V4,
            linode_id: 123,
            address: "string".to_string(),
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
        .ips()
        .create(linode_api::resources::linode::instances::ips::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdIpsApiVersionEnum::V4,
            linode_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdIpsBody {
                public: true,
                type_field: linode_api::models::PostApiVersionLinodeInstancesLinodeIdIpsBodyTypeEnum::Ipv4,
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
        .ips()
        .put(linode_api::resources::linode::instances::ips::PutRequest {
            api_version: linode_api::models::PutApiVersionLinodeInstancesLinodeIdIpsAddressApiVersionEnum::V4,
            linode_id: 123,
            address: "string".to_string(),
            data: linode_api::models::PutApiVersionLinodeInstancesLinodeIdIpsAddressBody {
                rdns: Some("test.example.org".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
