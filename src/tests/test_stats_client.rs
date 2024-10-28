#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .nodebalancers()
        .stats()
        .list(linode_api::resources::nodebalancers::stats::ListRequest {
            api_version: linode_api::models::GetApiVersionNodebalancersNodeBalancerIdStatsApiVersionEnum::V4,
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
        .managed()
        .stats()
        .list(linode_api::resources::managed::stats::ListRequest {
            api_version: linode_api::models::GetApiVersionManagedStatsApiVersionEnum::V4,
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
        .stats()
        .list(linode_api::resources::linode::instances::stats::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdStatsApiVersionEnum::V4,
            linode_id: 123,
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
        .stats()
        .get(linode_api::resources::linode::instances::stats::GetRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdStatsYearMonthApiVersionEnum::V4,
            linode_id: 123,
            year: 123,
            month: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
