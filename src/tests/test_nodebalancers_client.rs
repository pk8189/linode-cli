#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .nodebalancers()
        .delete(linode_api::resources::nodebalancers::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionNodebalancersNodeBalancerIdApiVersionEnum::V4,
            node_balancer_id: 123,
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
        .nodebalancers()
        .list(linode_api::resources::nodebalancers::ListRequest {
            api_version: linode_api::models::GetApiVersionNodebalancersApiVersionEnum::V4,
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
        .nodebalancers()
        .get(linode_api::resources::nodebalancers::GetRequest {
            api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdApiVersionEnum::V4,
            node_balancer_id: 123,
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
        .nodebalancers()
        .create(linode_api::resources::nodebalancers::CreateRequest {
            api_version: linode_api::models::PostApiVersionNodebalancersApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionNodebalancersBody {
                client_conn_throttle: Some(0),
                configs: Some(
                    vec![
                        linode_api::models::PostApiVersionNodebalancersBodyConfigsItem {
                        algorithm :
                        Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemAlgorithmEnum::Roundrobin),
                        check :
                        Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemCheckEnum::HttpBody),
                        check_attempts : Some(3), check_body : Some("it works"
                        .to_string()), check_interval : Some(90), check_passive :
                        Some(true), check_path : Some("/test".to_string()), check_timeout
                        : Some(10), cipher_suite :
                        Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemCipherSuiteEnum::Recommended),
                        nodes :
                        vec![linode_api::models::PostApiVersionNodebalancersBodyConfigsItemNodesItem
                        { address : Some("192.168.210.120:80".to_string()), config_id :
                        Some(4567), id : Some(54321), label : Some("node54321"
                        .to_string()), mode :
                        Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemNodesItemModeEnum::Accept),
                        nodebalancer_id : Some(12345), status :
                        Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemNodesItemStatusEnum::Up),
                        weight : Some(50), ..Default::default() }], port : Some(80),
                        protocol :
                        Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemProtocolEnum::Http),
                        proxy_protocol :
                        Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemProxyProtocolEnum::None),
                        ssl_cert : linode_api::Patch::new("<REDACTED>".to_string()),
                        ssl_key : linode_api::Patch::new("<REDACTED>".to_string()),
                        stickiness :
                        Some(linode_api::models::PostApiVersionNodebalancersBodyConfigsItemStickinessEnum::HttpCookie),
                        ..Default::default() }
                    ],
                ),
                firewall_id: Some(123),
                label: Some("balancer12345".to_string()),
                region: "us-east".to_string(),
                tags: Some(vec!["test".to_string(), "web-dev-team".to_string()]),
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
        .nodebalancers()
        .put(linode_api::resources::nodebalancers::PutRequest {
            api_version: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdApiVersionEnum::V4,
            node_balancer_id: 123,
            data: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdBody {
                client_conn_throttle: Some(0),
                created: Some("2018-01-01T00:01:01".to_string()),
                hostname: Some("192.0.2.1.ip.linodeusercontent.com".to_string()),
                id: Some(12345),
                ipv4: Some("203.0.113.1".to_string()),
                ipv6: linode_api::Patch::new("string".to_string()),
                label: Some("balancer12345".to_string()),
                region: Some("us-east".to_string()),
                tags: Some(
                    vec!["example tag".to_string(), "another example".to_string()],
                ),
                transfer: Some(linode_api::models::PutApiVersionNodebalancersNodeBalancerIdBodyTransfer {
                    in_field: linode_api::Patch::new(28.91200828552246),
                    out: linode_api::Patch::new(3.5487728118896484),
                    total: linode_api::Patch::new(32.46078109741211),
                    ..Default::default()
                }),
                updated: Some("2018-03-01T00:01:01".to_string()),
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
        .nodebalancers()
        .list(linode_api::resources::linode::instances::nodebalancers::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdNodebalancersApiVersionEnum::V4,
            linode_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
