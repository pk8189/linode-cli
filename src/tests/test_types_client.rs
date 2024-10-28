#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .volumes()
        .types_resource()
        .list(linode_api::resources::volumes::types_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionVolumesTypesApiVersionEnum::V4,
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
        .types_resource()
        .list(linode_api::resources::object_storage::types_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionObjectStorageTypesApiVersionEnum::V4,
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
        .nodebalancers()
        .types_resource()
        .list(linode_api::resources::nodebalancers::types_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionNodebalancersTypesApiVersionEnum::V4,
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
        .longview()
        .types_resource()
        .list(linode_api::resources::longview::types_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionLongviewTypesApiVersionEnum::V4,
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
        .lke()
        .types_resource()
        .list(linode_api::resources::lke::types_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionLkeTypesApiVersionEnum::V4,
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
        .linode()
        .types_resource()
        .list(linode_api::resources::linode::types_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeTypesApiVersionEnum::V4,
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
        .linode()
        .types_resource()
        .get(linode_api::resources::linode::types_resource::GetRequest {
            api_version: linode_api::models::GetApiVersionLinodeTypesTypeIdApiVersionEnum::V4,
            type_id: "string".to_string(),
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
        .types_resource()
        .list(linode_api::resources::databases::types_resource::ListRequest {
            api_version: linode_api::models::GetApiVersionDatabasesTypesApiVersionEnum::V4,
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
        .databases()
        .types_resource()
        .get(linode_api::resources::databases::types_resource::GetRequest {
            api_version: linode_api::models::GetApiVersionDatabasesTypesTypeIdApiVersionEnum::V4,
            type_id: "string".to_string(),
            ..Default::default()
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
