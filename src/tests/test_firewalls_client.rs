#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .nodebalancers()
        .firewalls()
        .list(linode_api::resources::nodebalancers::firewalls::ListRequest {
            api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdFirewallsApiVersionEnum::V4,
            node_balancer_id: 123,
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
        .delete(linode_api::resources::networking::firewalls::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionNetworkingFirewallsFirewallIdApiVersionEnum::V4,
            firewall_id: 123,
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
        .list(linode_api::resources::networking::firewalls::ListRequest {
            api_version: linode_api::models::GetApiVersionNetworkingFirewallsApiVersionEnum::V4,
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
        .get(linode_api::resources::networking::firewalls::GetRequest {
            api_version: linode_api::models::GetApiVersionNetworkingFirewallsFirewallIdApiVersionEnum::V4,
            firewall_id: 123,
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
        .create(linode_api::resources::networking::firewalls::CreateRequest {
            api_version: linode_api::models::PostApiVersionNetworkingFirewallsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionNetworkingFirewallsBody {
                created: Some("2018-01-01T00:01:01".to_string()),
                id: Some(123),
                label: Some("firewall123".to_string()),
                rules: Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRules {
                    fingerprint: Some("997dd135".to_string()),
                    inbound: Some(
                        vec![
                            linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesInboundItem
                            { action :
                            Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesInboundItemActionEnum::Accept),
                            addresses :
                            Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesInboundItemAddresses
                            { ipv4 : Some(vec!["192.0.2.0/24".to_string(),
                            "198.51.100.2/32".to_string()]), ipv6 :
                            Some(vec!["2001:DB8::/128".to_string()]),
                            ..Default::default() }), description :
                            Some("An example firewall rule description.".to_string()),
                            label : Some("firewallrule123".to_string()), ports :
                            linode_api::Patch::new("22-24, 80, 443".to_string()),
                            protocol :
                            Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesInboundItemProtocolEnum::Tcp),
                            ..Default::default() }
                        ],
                    ),
                    inbound_policy: Some(
                        linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesInboundPolicyEnum::Drop,
                    ),
                    outbound: Some(
                        vec![
                            linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesOutboundItem
                            { action :
                            Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesOutboundItemActionEnum::Accept),
                            addresses :
                            Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesOutboundItemAddresses
                            { ipv4 : Some(vec!["192.0.2.0/24".to_string(),
                            "198.51.100.2/32".to_string()]), ipv6 :
                            Some(vec!["2001:DB8::/128".to_string()]),
                            ..Default::default() }), description :
                            Some("An example firewall rule description.".to_string()),
                            label : Some("firewallrule123".to_string()), ports :
                            linode_api::Patch::new("22-24, 80, 443".to_string()),
                            protocol :
                            Some(linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesOutboundItemProtocolEnum::Tcp),
                            ..Default::default() }
                        ],
                    ),
                    outbound_policy: Some(
                        linode_api::models::PostApiVersionNetworkingFirewallsBodyRulesOutboundPolicyEnum::Drop,
                    ),
                    version: Some(1),
                    ..Default::default()
                }),
                status: Some(
                    linode_api::models::PostApiVersionNetworkingFirewallsBodyStatusEnum::Enabled,
                ),
                tags: Some(
                    vec!["example tag".to_string(), "another example".to_string()],
                ),
                updated: Some("2018-01-02T00:01:01".to_string()),
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
        .firewalls()
        .put(linode_api::resources::networking::firewalls::PutRequest {
            api_version: linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdApiVersionEnum::V4,
            firewall_id: 123,
            data: linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdBody {
                label: Some("firewall123".to_string()),
                status: Some(
                    linode_api::models::PutApiVersionNetworkingFirewallsFirewallIdBodyStatusEnum::Enabled,
                ),
                tags: Some(
                    vec!["example tag".to_string(), "another example".to_string()],
                ),
                ..Default::default()
            },
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
        .firewalls()
        .list(linode_api::resources::linode::instances::firewalls::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdFirewallsApiVersionEnum::V4,
            linode_id: 123,
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
