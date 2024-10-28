#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .vpcs()
        .delete(linode_api::resources::vpcs::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionVpcsVpcIdApiVersionEnum::V4,
            vpc_id: 123,
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
        .vpcs()
        .list(linode_api::resources::vpcs::ListRequest {
            api_version: linode_api::models::GetApiVersionVpcsApiVersionEnum::V4,
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
        .vpcs()
        .get(linode_api::resources::vpcs::GetRequest {
            api_version: linode_api::models::GetApiVersionVpcsVpcIdApiVersionEnum::V4,
            vpc_id: 123,
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
        .vpcs()
        .create(linode_api::resources::vpcs::CreateRequest {
            api_version: linode_api::models::PostApiVersionVpcsApiVersionEnum::V4,
            data: "could be anything",
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
        .vpcs()
        .put(linode_api::resources::vpcs::PutRequest {
            api_version: linode_api::models::PutApiVersionVpcsVpcIdApiVersionEnum::V4,
            vpc_id: 123,
            data: linode_api::models::PutApiVersionVpcsVpcIdBody {
                description: Some("A description of my VPC.".to_string()),
                label: Some("cool-vpc".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
