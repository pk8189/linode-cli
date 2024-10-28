#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .volumes()
        .attach()
        .create(linode_api::resources::volumes::attach::CreateRequest {
            api_version: linode_api::models::PostApiVersionVolumesVolumeIdAttachApiVersionEnum::V4,
            volume_id: 123,
            data: linode_api::models::PostApiVersionVolumesVolumeIdAttachBody {
                config_id: Some(23456),
                linode_id: 123,
                persist_across_boots: Some(true),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
