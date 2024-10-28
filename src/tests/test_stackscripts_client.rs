#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .linode()
        .stackscripts()
        .delete(linode_api::resources::linode::stackscripts::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLinodeStackscriptsStackscriptIdApiVersionEnum::V4,
            stackscript_id: "string".to_string(),
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
        .stackscripts()
        .list(linode_api::resources::linode::stackscripts::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeStackscriptsApiVersionEnum::V4,
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
        .linode()
        .stackscripts()
        .get(linode_api::resources::linode::stackscripts::GetRequest {
            api_version: linode_api::models::GetApiVersionLinodeStackscriptsStackscriptIdApiVersionEnum::V4,
            stackscript_id: "string".to_string(),
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
        .linode()
        .stackscripts()
        .create(linode_api::resources::linode::stackscripts::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeStackscriptsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionLinodeStackscriptsBody {
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
        .linode()
        .stackscripts()
        .put(linode_api::resources::linode::stackscripts::PutRequest {
            api_version: linode_api::models::PutApiVersionLinodeStackscriptsStackscriptIdApiVersionEnum::V4,
            stackscript_id: "string".to_string(),
            data: linode_api::models::PutApiVersionLinodeStackscriptsStackscriptIdBody {
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
