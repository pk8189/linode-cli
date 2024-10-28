#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .support()
        .tickets()
        .list(linode_api::resources::support::tickets::ListRequest {
            api_version: linode_api::models::GetApiVersionSupportTicketsApiVersionEnum::V4,
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
        .support()
        .tickets()
        .get(linode_api::resources::support::tickets::GetRequest {
            api_version: linode_api::models::GetApiVersionSupportTicketsTicketIdApiVersionEnum::V4,
            ticket_id: 123,
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
        .support()
        .tickets()
        .create(linode_api::resources::support::tickets::CreateRequest {
            api_version: linode_api::models::PostApiVersionSupportTicketsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionSupportTicketsBody {
                database_id: Some(123),
                description: "I'm having trouble setting the root password on my Linode. I tried following the instructions but something is not working and I'm not sure what I'm doing wrong. Can you please help me figure out how I can reset it?"
                    .to_string(),
                domain_id: Some(123),
                firewall_id: Some(123),
                linode_id: Some(123),
                lkecluster_id: Some(123),
                longviewclient_id: Some(123),
                managed_issue: Some(false),
                nodebalancer_id: Some(123),
                region: Some("string".to_string()),
                summary: "Having trouble resetting root password on my Linode"
                    .to_string(),
                vlan: Some("string".to_string()),
                volume_id: Some(123),
                vpc_id: Some(123),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
