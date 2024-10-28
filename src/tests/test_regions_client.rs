#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .regions()
        .list(linode_api::resources::regions::ListRequest {
            api_version: linode_api::models::GetApiVersionRegionsApiVersionEnum::V4,
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
        .regions()
        .get(linode_api::resources::regions::GetRequest {
            api_version: linode_api::models::GetApiVersionRegionsRegionIdApiVersionEnum::V4,
            region_id: "string".to_string(),
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
        .images()
        .regions()
        .create(linode_api::resources::images::regions::CreateRequest {
            api_version: linode_api::models::PostApiVersionImagesImageIdRegionsApiVersionEnum::V4,
            image_id: "linode/debian11".to_string(),
            data: linode_api::models::PostApiVersionImagesImageIdRegionsBody {
                regions: Some(vec!["us-iad".to_string(), "us-west".to_string()]),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
