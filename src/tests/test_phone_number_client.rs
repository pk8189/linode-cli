#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .profile_resource()
        .phone_number()
        .delete(linode_api::resources::profile_resource::phone_number::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionProfilePhoneNumberApiVersionEnum::V4,
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
        .profile_resource()
        .phone_number()
        .create(linode_api::resources::profile_resource::phone_number::CreateRequest {
            api_version: linode_api::models::PostApiVersionProfilePhoneNumberApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionProfilePhoneNumberBody {
                iso_code: "US".to_string(),
                phone_number: "555-555-5555".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
