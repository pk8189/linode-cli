#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .profile_resource()
        .security_questions()
        .list(linode_api::resources::profile_resource::security_questions::ListRequest {
            api_version: linode_api::models::GetApiVersionProfileSecurityQuestionsApiVersionEnum::V4,
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
        .security_questions()
        .create(linode_api::resources::profile_resource::security_questions::CreateRequest {
            api_version: linode_api::models::PostApiVersionProfileSecurityQuestionsApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionProfileSecurityQuestionsBody {
                security_questions: Some(
                    vec![
                        linode_api::models::PostApiVersionProfileSecurityQuestionsBodySecurityQuestionsItem
                        { question_id : Some(1), response : Some("Gotham City"
                        .to_string()), security_question :
                        Some("In what city were you born?".to_string()),
                        ..Default::default() }
                    ],
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
