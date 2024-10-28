#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .placement()
        .groups()
        .delete(linode_api::resources::placement::groups::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionPlacementGroupsGroupIdApiVersionEnum::V4,
            group_id: 123,
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
        .placement()
        .groups()
        .list(linode_api::resources::placement::groups::ListRequest {
            api_version: linode_api::models::GetApiVersionPlacementGroupsApiVersionEnum::V4,
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
        .placement()
        .groups()
        .get(linode_api::resources::placement::groups::GetRequest {
            api_version: linode_api::models::GetApiVersionPlacementGroupsGroupIdApiVersionEnum::V4,
            group_id: 123,
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
        .placement()
        .groups()
        .create(linode_api::resources::placement::groups::CreateRequest {
            api_version: linode_api::models::PostApiVersionPlacementGroupsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionPlacementGroupsBody {
                label: Some("PG_Miami_failover".to_string()),
                placement_group_policy: Some(
                    linode_api::models::PostApiVersionPlacementGroupsBodyPlacementGroupPolicyEnum::Strict,
                ),
                placement_group_type: Some(
                    linode_api::models::PostApiVersionPlacementGroupsBodyPlacementGroupTypeEnum::AntiAffinityLocal,
                ),
                region: Some("us-iad".to_string()),
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
        .placement()
        .groups()
        .put(linode_api::resources::placement::groups::PutRequest {
            api_version: linode_api::models::PutApiVersionPlacementGroupsGroupIdApiVersionEnum::V4,
            group_id: 123,
            data: linode_api::models::PutApiVersionPlacementGroupsGroupIdBody {
                label: Some("PG_Miami_failover".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
