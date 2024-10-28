#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .profile_resource()
        .grants()
        .list(linode_api::resources::profile_resource::grants::ListRequest {
            api_version: linode_api::models::GetApiVersionProfileGrantsApiVersionEnum::V4,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_204_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .profile_resource()
        .grants()
        .list(linode_api::resources::profile_resource::grants::ListRequest {
            api_version: linode_api::models::GetApiVersionProfileGrantsApiVersionEnum::V4,
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
        .account()
        .users()
        .grants()
        .list(linode_api::resources::account::users::grants::ListRequest {
            api_version: linode_api::models::GetApiVersionAccountUsersUsernameGrantsApiVersionEnum::V4,
            username: "string".to_string(),
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_list_204_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .account()
        .users()
        .grants()
        .list(linode_api::resources::account::users::grants::ListRequest {
            api_version: linode_api::models::GetApiVersionAccountUsersUsernameGrantsApiVersionEnum::V4,
            username: "string".to_string(),
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
        .account()
        .users()
        .grants()
        .put(linode_api::resources::account::users::grants::PutRequest {
            api_version: linode_api::models::PutApiVersionAccountUsersUsernameGrantsApiVersionEnum::V4,
            username: "string".to_string(),
            data: linode_api::models::PutApiVersionAccountUsersUsernameGrantsBody {
                database: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyDatabaseItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyDatabaseItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                domain: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyDomainItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyDomainItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                firewall: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyFirewallItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyFirewallItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                global: Some(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyGlobal {
                    account_access: linode_api::Patch::new(
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyGlobalAccountAccessEnum::ReadOnly,
                    ),
                    add_databases: Some(true),
                    add_domains: Some(true),
                    add_firewalls: Some(true),
                    add_images: Some(true),
                    add_linodes: Some(true),
                    add_loadbalancers: Some(true),
                    add_longview: Some(true),
                    add_nodebalancers: Some(true),
                    add_placement_groups: Some(true),
                    add_stackscripts: Some(true),
                    add_volumes: Some(true),
                    add_vpcs: Some(true),
                    cancel_account: Some(false),
                    child_account_access: linode_api::Patch::new(true),
                    longview_subscription: Some(true),
                    ..Default::default()
                }),
                image: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyImageItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyImageItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                linode: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyLinodeItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyLinodeItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                longview: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyLongviewItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyLongviewItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                nodebalancer: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyNodebalancerItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyNodebalancerItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                placement_group: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyPlacementGroupItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyPlacementGroupItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                stackscript: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyStackscriptItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyStackscriptItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                volume: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyVolumeItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyVolumeItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                vpc: Some(
                    vec![
                        linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyVpcItem
                        { id : Some(123), label : Some("example-entity".to_string()),
                        permissions :
                        linode_api::Patch::new(linode_api::models::PutApiVersionAccountUsersUsernameGrantsBodyVpcItemPermissionsEnum::ReadOnly),
                        ..Default::default() }
                    ],
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
