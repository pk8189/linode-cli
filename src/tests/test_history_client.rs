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
        .history()
        .list(linode_api::resources::networking::firewalls::history::ListRequest {
            api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryApiVersionEnum::V4,
            firewall_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
