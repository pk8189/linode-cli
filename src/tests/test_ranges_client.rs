#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .networking()
        .ipv6()
        .ranges()
        .delete(linode_api::resources::networking::ipv6::ranges::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionNetworkingIpv6RangesRangeApiVersionEnum::V4,
            range: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(),
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
        .ipv6()
        .ranges()
        .list(linode_api::resources::networking::ipv6::ranges::ListRequest {
            api_version: linode_api::models::GetApiVersionNetworkingIpv6RangesApiVersionEnum::V4,
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
        .ipv6()
        .ranges()
        .get(linode_api::resources::networking::ipv6::ranges::GetRequest {
            api_version: linode_api::models::GetApiVersionNetworkingIpv6RangesRangeApiVersionEnum::V4,
            range: "2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string(),
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
        .ipv6()
        .ranges()
        .create(linode_api::resources::networking::ipv6::ranges::CreateRequest {
            api_version: linode_api::models::PostApiVersionNetworkingIpv6RangesApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionNetworkingIpv6RangesBody {
                linode_id: Some(123),
                prefix_length: 123,
                route_target: Some("2001:0db8::1".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
