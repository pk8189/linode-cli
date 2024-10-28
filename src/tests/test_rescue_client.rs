#[serial_test::serial]
#[tokio::test]
async fn test_create_200_success_default() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .linode()
        .instances()
        .rescue()
        .create(linode_api::resources::linode::instances::rescue::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueApiVersionEnum::V4,
            linode_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBody {
                devices: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevices {
                    sda: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSda {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdb: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdb {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdc: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdc {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdd: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdd {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sde: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSde {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdf: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdf {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    sdg: Some(linode_api::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdg {
                        disk_id: Some(124458),
                        volume_id: Some(123),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
