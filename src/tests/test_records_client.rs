#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .domains()
        .records()
        .delete(linode_api::resources::domains::records::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionDomainsDomainIdRecordsRecordIdApiVersionEnum::V4,
            domain_id: 123,
            record_id: 123,
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
        .records()
        .list(linode_api::resources::domains::records::ListRequest {
            api_version: linode_api::models::GetApiVersionDomainsDomainIdRecordsApiVersionEnum::V4,
            domain_id: 123,
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
        .records()
        .get(linode_api::resources::domains::records::GetRequest {
            api_version: linode_api::models::GetApiVersionDomainsDomainIdRecordsRecordIdApiVersionEnum::V4,
            domain_id: 123,
            record_id: 123,
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
        .records()
        .create(linode_api::resources::domains::records::CreateRequest {
            api_version: linode_api::models::PostApiVersionDomainsDomainIdRecordsApiVersionEnum::V4,
            domain_id: 123,
            data: linode_api::models::PostApiVersionDomainsDomainIdRecordsBody {
                created: Some("2018-01-01T00:01:01".to_string()),
                id: Some(123456),
                name: Some("test".to_string()),
                port: Some(80),
                priority: Some(50),
                protocol: linode_api::Patch::new("string".to_string()),
                service: linode_api::Patch::new("string".to_string()),
                tag: linode_api::Patch::new(
                    linode_api::models::PostApiVersionDomainsDomainIdRecordsBodyTagEnum::Iodef,
                ),
                target: Some("192.0.2.0".to_string()),
                ttl_sec: Some(604800),
                type_field: Some(
                    linode_api::models::PostApiVersionDomainsDomainIdRecordsBodyTypeEnum::A,
                ),
                updated: Some("2018-01-01T00:01:01".to_string()),
                weight: Some(50),
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
        .records()
        .put(linode_api::resources::domains::records::PutRequest {
            api_version: linode_api::models::PutApiVersionDomainsDomainIdRecordsRecordIdApiVersionEnum::V4,
            domain_id: 123,
            record_id: 123,
            data: linode_api::models::PutApiVersionDomainsDomainIdRecordsRecordIdBody {
                name: Some("test".to_string()),
                port: Some(80),
                priority: Some(50),
                protocol: linode_api::Patch::new("string".to_string()),
                service: linode_api::Patch::new("string".to_string()),
                tag: linode_api::Patch::new(
                    linode_api::models::PutApiVersionDomainsDomainIdRecordsRecordIdBodyTagEnum::Iodef,
                ),
                target: Some("192.0.2.0".to_string()),
                ttl_sec: Some(604800),
                weight: Some(50),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
