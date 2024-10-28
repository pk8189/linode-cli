#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .domains()
        .delete(linode_api::resources::domains::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionDomainsDomainIdApiVersionEnum::V4,
            domain_id: 123,
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
        .domains()
        .list(linode_api::resources::domains::ListRequest {
            api_version: linode_api::models::GetApiVersionDomainsApiVersionEnum::V4,
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
        .domains()
        .get(linode_api::resources::domains::GetRequest {
            api_version: linode_api::models::GetApiVersionDomainsDomainIdApiVersionEnum::V4,
            domain_id: 123,
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
        .domains()
        .create(linode_api::resources::domains::CreateRequest {
            api_version: linode_api::models::PostApiVersionDomainsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionDomainsBody {
                axfr_ips: Some(vec!["string".to_string()]),
                description: Some("string".to_string()),
                domain: Some("example.org".to_string()),
                expire_sec: Some(300),
                group: Some("string".to_string()),
                id: Some(1234),
                master_ips: Some(vec!["string".to_string()]),
                refresh_sec: Some(300),
                retry_sec: Some(300),
                soa_email: Some("admin@example.org".to_string()),
                status: Some(
                    linode_api::models::PostApiVersionDomainsBodyStatusEnum::Active,
                ),
                tags: Some(
                    vec!["example tag".to_string(), "another example".to_string()],
                ),
                ttl_sec: Some(300),
                type_field: Some(
                    linode_api::models::PostApiVersionDomainsBodyTypeEnum::Master,
                ),
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
        .domains()
        .put(linode_api::resources::domains::PutRequest {
            api_version: linode_api::models::PutApiVersionDomainsDomainIdApiVersionEnum::V4,
            domain_id: 123,
            data: linode_api::models::PutApiVersionDomainsDomainIdBody {
                axfr_ips: Some(vec!["string".to_string()]),
                description: Some("string".to_string()),
                domain: Some("example.org".to_string()),
                expire_sec: Some(300),
                group: Some("string".to_string()),
                id: Some(1234),
                master_ips: Some(vec!["string".to_string()]),
                refresh_sec: Some(300),
                retry_sec: Some(300),
                soa_email: Some("admin@example.org".to_string()),
                status: Some(
                    linode_api::models::PutApiVersionDomainsDomainIdBodyStatusEnum::Active,
                ),
                tags: Some(
                    vec!["example tag".to_string(), "another example".to_string()],
                ),
                ttl_sec: Some(300),
                type_field: Some(
                    linode_api::models::PutApiVersionDomainsDomainIdBodyTypeEnum::Master,
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
