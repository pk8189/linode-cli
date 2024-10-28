#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .vpcs()
        .subnets()
        .delete(linode_api::resources::vpcs::subnets::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionVpcsVpcIdSubnetsVpcSubnetIdApiVersionEnum::V4,
            vpc_id: 123,
            vpc_subnet_id: 123,
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
        .subnets()
        .list(linode_api::resources::vpcs::subnets::ListRequest {
            api_version: linode_api::models::GetApiVersionVpcsVpcIdSubnetsApiVersionEnum::V4,
            vpc_id: 123,
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
        .subnets()
        .get(linode_api::resources::vpcs::subnets::GetRequest {
            api_version: linode_api::models::GetApiVersionVpcsVpcIdSubnetsVpcSubnetIdApiVersionEnum::V4,
            vpc_id: 123,
            vpc_subnet_id: 123,
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
        .subnets()
        .create(linode_api::resources::vpcs::subnets::CreateRequest {
            api_version: linode_api::models::PostApiVersionVpcsVpcIdSubnetsApiVersionEnum::V4,
            vpc_id: 123,
            data: linode_api::models::PostApiVersionVpcsVpcIdSubnetsBody {
                ipv4: "10.0.1.0/24".to_string(),
                label: "cool-vpc-subnet".to_string(),
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
        .vpcs()
        .subnets()
        .put(linode_api::resources::vpcs::subnets::PutRequest {
            api_version: linode_api::models::PutApiVersionVpcsVpcIdSubnetsVpcSubnetIdApiVersionEnum::V4,
            vpc_id: 123,
            vpc_subnet_id: 123,
            data: linode_api::models::PutApiVersionVpcsVpcIdSubnetsVpcSubnetIdBody {
                label: Some("cool-vpc-subnet".to_string()),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
