#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .tags()
        .delete(linode_api::resources::tags::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionTagsTagLabelApiVersionEnum::V4,
            tag_label: "string".to_string(),
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
        .tags()
        .list(linode_api::resources::tags::ListRequest {
            api_version: linode_api::models::GetApiVersionTagsApiVersionEnum::V4,
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
        .tags()
        .get(linode_api::resources::tags::GetRequest {
            api_version: linode_api::models::GetApiVersionTagsTagLabelApiVersionEnum::V4,
            tag_label: "string".to_string(),
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
        .tags()
        .create(linode_api::resources::tags::CreateRequest {
            api_version: linode_api::models::PostApiVersionTagsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionTagsBody {
                domains: Some(vec![564, 565]),
                label: "example tag".to_string(),
                linodes: Some(vec![123, 456]),
                nodebalancers: Some(vec![10301, 10501]),
                volumes: Some(vec![9082, 10003]),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
