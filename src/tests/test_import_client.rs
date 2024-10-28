#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .domains()
        .import_resource()
        .create(linode_api::resources::domains::import_resource::CreateRequest {
            api_version: linode_api::models::PostApiVersionDomainsImportApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionDomainsImportBody {
                domain: "example.com".to_string(),
                remote_nameserver: "examplenameserver.com".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
