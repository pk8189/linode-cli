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
        .backups()
        .restore()
        .create(linode_api::resources::linode::instances::backups::restore::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsBackupIdRestoreApiVersionEnum::V4,
            linode_id: 123,
            backup_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsBackupIdRestoreBody {
                linode_id: 234,
                overwrite: Some(true),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .databases()
        .postgresql()
        .instances()
        .backups()
        .restore()
        .create(linode_api::resources::databases::postgresql::instances::backups::restore::CreateRequest {
            api_version: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBackupIdRestoreApiVersionEnum::V4,
            instance_id: 123,
            backup_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_create_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .databases()
        .mysql()
        .instances()
        .backups()
        .restore()
        .create(linode_api::resources::databases::mysql::instances::backups::restore::CreateRequest {
            api_version: linode_api::models::PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBackupIdRestoreApiVersionEnum::V4,
            instance_id: 123,
            backup_id: 123,
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
