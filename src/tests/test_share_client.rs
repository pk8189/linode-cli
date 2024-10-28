#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .networking()
        .ipv4()
        .share()
        .create(linode_api::resources::networking::ipv4::share::CreateRequest {
            api_version: linode_api::models::PostApiVersionNetworkingIpv4ShareApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionNetworkingIpv4ShareBody {
                ips: vec!["192.0.2.1".to_string(), "2001:db8:3c4d:15::".to_string()],
                linode_id: 123,
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
        .networking()
        .ips()
        .share()
        .create(linode_api::resources::networking::ips::share::CreateRequest {
            api_version: linode_api::models::PostApiVersionNetworkingIpsShareApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionNetworkingIpsShareBody {
                ips: vec!["192.0.2.1".to_string(), "2001:db8:3c4d:15::".to_string()],
                linode_id: 123,
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
