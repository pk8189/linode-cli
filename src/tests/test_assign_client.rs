#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .placement()
        .groups()
        .assign()
        .create(linode_api::resources::placement::groups::assign::CreateRequest {
            api_version: linode_api::models::PostApiVersionPlacementGroupsGroupIdAssignApiVersionEnum::V4,
            group_id: 123,
            data: linode_api::models::PostApiVersionPlacementGroupsGroupIdAssignBody {
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
        .ipv4()
        .assign()
        .create(linode_api::resources::networking::ipv4::assign::CreateRequest {
            api_version: linode_api::models::PostApiVersionNetworkingIpv4AssignApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionNetworkingIpv4AssignBody {
                assignments: vec![
                    linode_api::models::PostApiVersionNetworkingIpv4AssignBodyAssignmentsItem
                    { address : "192.0.2.1".to_string(), linode_id : 123,
                    ..Default::default() }
                ],
                region: "us-east".to_string(),
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
        .assign()
        .create(linode_api::resources::networking::ips::assign::CreateRequest {
            api_version: linode_api::models::PostApiVersionNetworkingIpsAssignApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionNetworkingIpsAssignBody {
                assignments: vec![
                    linode_api::models::PostApiVersionNetworkingIpsAssignBodyAssignmentsItem
                    { address : "192.0.2.1".to_string(), linode_id : 123,
                    ..Default::default() }
                ],
                region: "us-east".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
