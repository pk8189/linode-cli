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
        .replies()
        .list(linode_api::resources::support::tickets::replies::ListRequest {
            api_version: linode_api::models::GetApiVersionSupportTicketsTicketIdRepliesApiVersionEnum::V4,
            ticket_id: 123,
            ..Default::default()
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
        .replies()
        .create(linode_api::resources::support::tickets::replies::CreateRequest {
            api_version: linode_api::models::PostApiVersionSupportTicketsTicketIdRepliesApiVersionEnum::V4,
            ticket_id: 123,
            data: linode_api::models::PostApiVersionSupportTicketsTicketIdRepliesBody {
                description: "Thank you for your help. I was able to figure out what the problem was and I successfully reset my password. You guys are the best!"
                    .to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
