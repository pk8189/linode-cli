#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .nodebalancers()
        .configs()
        .delete(linode_api::resources::nodebalancers::configs::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionNodebalancersNodeBalancerIdConfigsConfigIdApiVersionEnum::V4,
            node_balancer_id: 123,
            config_id: 123,
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
        .configs()
        .list(linode_api::resources::nodebalancers::configs::ListRequest {
            api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdConfigsApiVersionEnum::V4,
            node_balancer_id: 123,
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
        .configs()
        .get(linode_api::resources::nodebalancers::configs::GetRequest {
            api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdApiVersionEnum::V4,
            node_balancer_id: 123,
            config_id: 123,
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
        .configs()
        .create(linode_api::resources::nodebalancers::configs::CreateRequest {
            api_version: linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsApiVersionEnum::V4,
            node_balancer_id: 123,
            data: linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBody {
                algorithm: Some(
                    linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyAlgorithmEnum::Roundrobin,
                ),
                check: Some(
                    linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyCheckEnum::HttpBody,
                ),
                check_attempts: Some(3),
                check_body: Some("it works".to_string()),
                check_interval: Some(90),
                check_passive: Some(true),
                check_path: Some("/test".to_string()),
                check_timeout: Some(10),
                cipher_suite: Some(
                    linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyCipherSuiteEnum::Recommended,
                ),
                id: Some(4567),
                nodebalancer_id: Some(12345),
                nodes_status: Some(linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyNodesStatus {
                    down: Some(0),
                    up: Some(4),
                    ..Default::default()
                }),
                port: Some(80),
                protocol: Some(
                    linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyProtocolEnum::Http,
                ),
                proxy_protocol: Some(
                    linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyProxyProtocolEnum::None,
                ),
                ssl_cert: linode_api::Patch::new("<REDACTED>".to_string()),
                ssl_commonname: Some("www.example.com".to_string()),
                ssl_fingerprint: Some(
                    "00:01:02:03:04:05:06:07:08:09:0A:0B:0C:0D:0E:0F:10:11:12:13"
                        .to_string(),
                ),
                ssl_key: linode_api::Patch::new("<REDACTED>".to_string()),
                stickiness: Some(
                    linode_api::models::PostApiVersionNodebalancersNodeBalancerIdConfigsBodyStickinessEnum::HttpCookie,
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
async fn test_put_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .nodebalancers()
        .configs()
        .put(linode_api::resources::nodebalancers::configs::PutRequest {
            api_version: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdApiVersionEnum::V4,
            node_balancer_id: 123,
            config_id: 123,
            data: linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBody {
                algorithm: Some(
                    linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyAlgorithmEnum::Roundrobin,
                ),
                check: Some(
                    linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyCheckEnum::HttpBody,
                ),
                check_attempts: Some(3),
                check_body: Some("it works".to_string()),
                check_interval: Some(90),
                check_passive: Some(true),
                check_path: Some("/test".to_string()),
                check_timeout: Some(10),
                cipher_suite: Some(
                    linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyCipherSuiteEnum::Recommended,
                ),
                id: Some(4567),
                nodebalancer_id: Some(12345),
                nodes_status: Some(linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyNodesStatus {
                    down: Some(0),
                    up: Some(4),
                    ..Default::default()
                }),
                port: Some(80),
                protocol: Some(
                    linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyProtocolEnum::Http,
                ),
                proxy_protocol: Some(
                    linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyProxyProtocolEnum::None,
                ),
                ssl_cert: linode_api::Patch::new("<REDACTED>".to_string()),
                ssl_commonname: Some("www.example.com".to_string()),
                ssl_fingerprint: Some(
                    "00:01:02:03:04:05:06:07:08:09:0A:0B:0C:0D:0E:0F:10:11:12:13"
                        .to_string(),
                ),
                ssl_key: linode_api::Patch::new("<REDACTED>".to_string()),
                stickiness: Some(
                    linode_api::models::PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdBodyStickinessEnum::HttpCookie,
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
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .linode()
        .instances()
        .configs()
        .delete(linode_api::resources::linode::instances::configs::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLinodeInstancesLinodeIdConfigsConfigIdApiVersionEnum::V4,
            linode_id: 123,
            config_id: 123,
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
        .list(linode_api::resources::linode::instances::configs::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdConfigsApiVersionEnum::V4,
            linode_id: 123,
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
        .linode()
        .instances()
        .configs()
        .get(linode_api::resources::linode::instances::configs::GetRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdApiVersionEnum::V4,
            linode_id: 123,
            config_id: 123,
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
        .create(linode_api::resources::linode::instances::configs::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsApiVersionEnum::V4,
            linode_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBody {
                comments: linode_api::Patch::new("This is my main Config".to_string()),
                devices: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevices {
                    sda: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSda {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdb: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdb {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdc: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdc {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdd: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdd {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sde: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSde {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdf: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdf {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdg: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdg {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdh: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdh {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                helpers: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyHelpers {
                    devtmpfs_automount: Some(false),
                    distro: Some(true),
                    modules_dep: Some(true),
                    network: Some(true),
                    updatedb_disabled: Some(true),
                    ..Default::default()
                }),
                id: Some(23456),
                interfaces: Some(
                    vec![
                        linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItem
                        { id : Some(101), ipam_address : linode_api::Patch::new(None),
                        ipv4 :
                        Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemIpv4
                        { ..Default::default() }), label : linode_api::Patch::new(None),
                        primary : Some(false), purpose :
                        linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum::Public,
                        subnet_id : linode_api::Patch::new(None), vpc_id :
                        linode_api::Patch::new(None), ..Default::default() },
                        linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItem
                        { id : Some(102), ipam_address :
                        linode_api::Patch::new("10.0.0.1/24".to_string()), ipv4 :
                        Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemIpv4
                        { nat_1_1 : linode_api::Patch::new(None), vpc :
                        linode_api::Patch::new("10.0.0.2".to_string()),
                        ..Default::default() }), label : linode_api::Patch::new("vlan-1"
                        .to_string()), primary : Some(false), purpose :
                        linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum::Vlan,
                        subnet_id : linode_api::Patch::new(None), vpc_id :
                        linode_api::Patch::new(None), ..Default::default() },
                        linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItem
                        { id : Some(103), ipam_address : linode_api::Patch::new(None),
                        ipv4 :
                        Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemIpv4
                        { nat_1_1 : linode_api::Patch::new("203.0.113.2".to_string()),
                        vpc : linode_api::Patch::new("10.0.1.2".to_string()),
                        ..Default::default() }), label : linode_api::Patch::new(None),
                        primary : Some(true), purpose :
                        linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum::Vpc,
                        subnet_id : linode_api::Patch::new(101), vpc_id :
                        linode_api::Patch::new(111), ..Default::default() }
                    ],
                ),
                kernel: Some("linode/latest-64bit".to_string()),
                label: Some("My Config".to_string()),
                memory_limit: Some(2048),
                root_device: Some("/dev/sda".to_string()),
                run_level: Some(
                    linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyRunLevelEnum::Default,
                ),
                virt_mode: Some(
                    linode_api::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyVirtModeEnum::Paravirt,
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
async fn test_put_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .linode()
        .instances()
        .configs()
        .put(linode_api::resources::linode::instances::configs::PutRequest {
            api_version: linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdApiVersionEnum::V4,
            linode_id: 123,
            config_id: 123,
            data: linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBody {
                comments: linode_api::Patch::new("This is my main Config".to_string()),
                devices: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevices {
                    sda: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSda {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdb: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdb {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdc: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdc {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdd: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdd {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sde: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSde {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdf: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdf {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdg: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdg {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdh: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdh {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                helpers: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyHelpers {
                    devtmpfs_automount: Some(false),
                    distro: Some(true),
                    modules_dep: Some(true),
                    network: Some(true),
                    updatedb_disabled: Some(true),
                    ..Default::default()
                }),
                id: Some(23456),
                interfaces: Some(
                    vec![
                        linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItem
                        { id : Some(101), ipam_address : linode_api::Patch::new(None),
                        ipv4 :
                        Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemIpv4
                        { ..Default::default() }), label : linode_api::Patch::new(None),
                        primary : Some(false), purpose :
                        linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemPurposeEnum::Public,
                        subnet_id : linode_api::Patch::new(None), vpc_id :
                        linode_api::Patch::new(None), ..Default::default() },
                        linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItem
                        { id : Some(102), ipam_address :
                        linode_api::Patch::new("10.0.0.1/24".to_string()), ipv4 :
                        Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemIpv4
                        { nat_1_1 : linode_api::Patch::new(None), vpc :
                        linode_api::Patch::new("10.0.0.2".to_string()),
                        ..Default::default() }), label : linode_api::Patch::new("vlan-1"
                        .to_string()), primary : Some(false), purpose :
                        linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemPurposeEnum::Vlan,
                        subnet_id : linode_api::Patch::new(None), vpc_id :
                        linode_api::Patch::new(None), ..Default::default() },
                        linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItem
                        { id : Some(103), ipam_address : linode_api::Patch::new(None),
                        ipv4 :
                        Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemIpv4
                        { nat_1_1 : linode_api::Patch::new("203.0.113.2".to_string()),
                        vpc : linode_api::Patch::new("10.0.1.2".to_string()),
                        ..Default::default() }), label : linode_api::Patch::new(None),
                        primary : Some(true), purpose :
                        linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyInterfacesItemPurposeEnum::Vpc,
                        subnet_id : linode_api::Patch::new(101), vpc_id :
                        linode_api::Patch::new(111), ..Default::default() }
                    ],
                ),
                kernel: Some("linode/latest-64bit".to_string()),
                label: Some("My Config".to_string()),
                memory_limit: Some(2048),
                root_device: Some("/dev/sda".to_string()),
                run_level: Some(
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyRunLevelEnum::Default,
                ),
                virt_mode: Some(
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyVirtModeEnum::Paravirt,
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
