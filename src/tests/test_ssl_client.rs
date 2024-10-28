#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .object_storage()
        .buckets()
        .ssl_resource()
        .delete(linode_api::resources::object_storage::buckets::ssl_resource::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionObjectStorageBucketsRegionIdBucketSslApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
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
        .object_storage()
        .buckets()
        .ssl_resource()
        .list(linode_api::resources::object_storage::buckets::ssl_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageBucketsRegionIdBucketSslApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
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
        .object_storage()
        .buckets()
        .ssl_resource()
        .create(linode_api::resources::object_storage::buckets::ssl_resource::CreateRequest {
            api_version: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketSslApiVersionEnum::V4,
            region_id: "string".to_string(),
            bucket: "string".to_string(),
            data: linode_api::models::PostApiVersionObjectStorageBucketsRegionIdBucketSslBody {
                certificate: "-----BEGIN CERTIFICATE-----\nCERTIFICATE_INFORMATION\n-----END CERTIFICATE-----"
                    .to_string(),
                private_key: "-----BEGIN PRIVATE KEY-----\nPRIVATE_KEY_INFORMATION\n-----END PRIVATE KEY-----"
                    .to_string(),
                ..Default::default()
            },
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
        .databases()
        .postgresql()
        .instances()
        .ssl_resource()
        .list(linode_api::resources::databases::postgresql::instances::ssl_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdSslApiVersionEnum::V4,
            instance_id: 123,
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
        .databases()
        .mysql()
        .instances()
        .ssl_resource()
        .list(linode_api::resources::databases::mysql::instances::ssl_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesInstanceIdSslApiVersionEnum::V4,
            instance_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
