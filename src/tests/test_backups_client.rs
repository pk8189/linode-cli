#[serial_test::serial]
#[tokio::test]
async fn test_list_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .linode()
        .instances()
        .backups()
        .list(linode_api::resources::linode::instances::backups::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdBackupsApiVersionEnum::V4,
            linode_id: 123,
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
        .instances()
        .backups()
        .get(linode_api::resources::linode::instances::backups::GetRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdBackupsBackupIdApiVersionEnum::V4,
            linode_id: 123,
            backup_id: 123,
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
        .instances()
        .backups()
        .create(linode_api::resources::linode::instances::backups::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsApiVersionEnum::V4,
            linode_id: 123,
            data: linode_api::models::PostApiVersionLinodeInstancesLinodeIdBackupsBody {
                label: "SnapshotLabel".to_string(),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .databases()
        .postgresql()
        .instances()
        .backups()
        .delete(linode_api::resources::databases::postgresql::instances::backups::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBackupIdApiVersionEnum::V4,
            instance_id: 123,
            backup_id: 123,
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
        .backups()
        .list(linode_api::resources::databases::postgresql::instances::backups::ListRequest {
            api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsApiVersionEnum::V4,
            instance_id: 123,
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
        .postgresql()
        .instances()
        .backups()
        .get(linode_api::resources::databases::postgresql::instances::backups::GetRequest {
            api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBackupIdApiVersionEnum::V4,
            instance_id: 123,
            backup_id: 123,
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
        .databases()
        .postgresql()
        .instances()
        .backups()
        .create(linode_api::resources::databases::postgresql::instances::backups::CreateRequest {
            api_version: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsApiVersionEnum::V4,
            instance_id: 123,
            data: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBody {
                label: "db-snapshot".to_string(),
                target: Some(
                    linode_api::models::PostApiVersionDatabasesPostgresqlInstancesInstanceIdBackupsBodyTargetEnum::Primary,
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .databases()
        .mysql()
        .instances()
        .backups()
        .delete(linode_api::resources::databases::mysql::instances::backups::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionDatabasesMysqlInstancesInstanceIdBackupsBackupIdApiVersionEnum::V4,
            instance_id: 123,
            backup_id: 123,
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
        .backups()
        .list(linode_api::resources::databases::mysql::instances::backups::ListRequest {
            api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesInstanceIdBackupsApiVersionEnum::V4,
            instance_id: 123,
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
        .mysql()
        .instances()
        .backups()
        .get(linode_api::resources::databases::mysql::instances::backups::GetRequest {
            api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesInstanceIdBackupsBackupIdApiVersionEnum::V4,
            instance_id: 123,
            backup_id: 123,
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
        .databases()
        .mysql()
        .instances()
        .backups()
        .create(linode_api::resources::databases::mysql::instances::backups::CreateRequest {
            api_version: linode_api::models::PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsApiVersionEnum::V4,
            instance_id: 123,
            data: linode_api::models::PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBody {
                label: "db-snapshot".to_string(),
                target: Some(
                    linode_api::models::PostApiVersionDatabasesMysqlInstancesInstanceIdBackupsBodyTargetEnum::Primary,
                ),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
