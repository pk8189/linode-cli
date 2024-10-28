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
        .rules()
        .list(linode_api::resources::networking::firewalls::rules::ListRequest {
            api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdRulesApiVersionEnum::V4,
            firewall_id: 123,
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
        .firewalls()
        .rules()
        .put(linode_api::resources::networking::firewalls::rules::PutRequest {
            api_version: linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesApiVersionEnum::V4,
            firewall_id: 123,
            data: linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBody {
                fingerprint: Some("997dd135".to_string()),
                inbound: Some(
                    vec![
                        linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyInboundItem
                        { action :
                        Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyInboundItemActionEnum::Accept),
                        addresses :
                        Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyInboundItemAddresses
                        { ipv4 : Some(vec!["192.0.2.0/24".to_string(), "198.51.100.2/32"
                        .to_string()]), ipv6 : Some(vec!["2001:DB8::/128".to_string()]),
                        ..Default::default() }), description :
                        Some("An example firewall rule description.".to_string()), label
                        : Some("firewallrule123".to_string()), ports :
                        linode_api::Patch::new("22-24, 80, 443".to_string()), protocol :
                        Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyInboundItemProtocolEnum::Tcp),
                        ..Default::default() }
                    ],
                ),
                inbound_policy: Some(
                    linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyInboundPolicyEnum::Drop,
                ),
                outbound: Some(
                    vec![
                        linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyOutboundItem
                        { action :
                        Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyOutboundItemActionEnum::Accept),
                        addresses :
                        Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyOutboundItemAddresses
                        { ipv4 : Some(vec!["192.0.2.0/24".to_string(), "198.51.100.2/32"
                        .to_string()]), ipv6 : Some(vec!["2001:DB8::/128".to_string()]),
                        ..Default::default() }), description :
                        Some("An example firewall rule description.".to_string()), label
                        : Some("firewallrule123".to_string()), ports :
                        linode_api::Patch::new("22-24, 80, 443".to_string()), protocol :
                        Some(linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyOutboundItemProtocolEnum::Tcp),
                        ..Default::default() }
                    ],
                ),
                outbound_policy: Some(
                    linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdRulesBodyOutboundPolicyEnum::Drop,
                ),
                version: Some(1),
                ..Default::default()
            },
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
        .history()
        .rules()
        .get(linode_api::resources::networking::firewalls::history::rules::GetRequest {
            api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdHistoryRulesVersionApiVersionEnum::V4,
            firewall_id: 123,
            version: 3,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
