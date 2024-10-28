#[serial_test::serial]
#[tokio::test]
async fn test_delete_200_generated_success() {
    let client = linode_api::Client::default()
        .with_oauth("API_TOKEN")
        .with_personal_access_token("API_TOKEN")
        .with_base_url("http://127.0.0.1:8082/v1/mock/local/linode/0.1.2");
    let res = client
        .linode()
        .instances()
        .delete(linode_api::resources::linode::instances::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionLinodeInstancesLinodeIdApiVersionEnum::V4,
            linode_id: 123,
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
        .instances()
        .list(linode_api::resources::linode::instances::ListRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesApiVersionEnum::V4,
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
        .instances()
        .get(linode_api::resources::linode::instances::GetRequest {
            api_version: linode_api::models::GetApiVersionLinodeInstancesLinodeIdApiVersionEnum::V4,
            linode_id: 123,
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
        .create(linode_api::resources::linode::instances::CreateRequest {
            api_version: linode_api::models::PostApiVersionLinodeInstancesApiVersionEnum::V4,
            data: "could be anything",
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
        .instances()
        .put(linode_api::resources::linode::instances::PutRequest {
            api_version: linode_api::models::PutApiVersionLinodeInstancesLinodeIdApiVersionEnum::V4,
            linode_id: 123,
            data: linode_api::models::PutApiVersionLinodeInstancesLinodeIdBody {
                alerts: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyAlerts {
                    cpu: Some(180),
                    io: Some(10000),
                    network_in: Some(10),
                    network_out: Some(10),
                    transfer_quota: Some(80),
                    ..Default::default()
                }),
                backups: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyBackups {
                    available: Some(true),
                    enabled: Some(true),
                    last_successful: Some("2018-01-01T00:01:01".to_string()),
                    schedule: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyBackupsSchedule {
                        day: linode_api::Patch::new(
                            linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyBackupsScheduleDayEnum::Saturday,
                        ),
                        window: linode_api::Patch::new(
                            linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyBackupsScheduleWindowEnum::W22,
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                capabilities: Some(vec!["Block Storage Encryption".to_string()]),
                created: Some("2018-01-01T00:01:01".to_string()),
                disk_encryption: linode_api::Patch::new("disabled".to_string()),
                group: Some("Linode-Group".to_string()),
                has_user_data: Some(true),
                host_uuid: Some("3a3ddd59d9a78bb8de041391075df44de62bfec8".to_string()),
                hypervisor: Some(
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyHypervisorEnum::Kvm,
                ),
                id: Some(123),
                image: Some("linode/debian9".to_string()),
                ipv4: Some(vec!["203.0.113.1".to_string(), "192.0.2.1".to_string()]),
                ipv6: linode_api::Patch::new("c001:d00d::1337/128".to_string()),
                label: Some("linode123".to_string()),
                lke_cluster_id: linode_api::Patch::new(1),
                placement_group: linode_api::Patch::new(linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroup {
                    id: Some(528),
                    label: Some("PG_Miami_failover".to_string()),
                    placement_group_policy: Some(
                        linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroupPlacementGroupPolicyEnum::Strict,
                    ),
                    placement_group_type: Some(
                        linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyPlacementGroupPlacementGroupTypeEnum::AntiAffinityLocal,
                    ),
                    ..Default::default()
                }),
                region: Some("us-east".to_string()),
                specs: Some(linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodySpecs {
                    disk: Some(81920),
                    gpus: Some(0),
                    memory: Some(4096),
                    transfer: Some(4000),
                    vcpus: Some(2),
                    ..Default::default()
                }),
                status: Some(
                    linode_api::models::PutApiVersionLinodeInstancesLinodeIdBodyStatusEnum::Running,
                ),
                tags: Some(
                    vec!["example tag".to_string(), "another example".to_string()],
                ),
                type_field: Some("g6-standard-1".to_string()),
                updated: Some("2018-01-01T00:01:01".to_string()),
                watchdog_enabled: Some(true),
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
        .instances()
        .list(linode_api::resources::databases::instances::ListRequest {
            api_version: linode_api::models::GetApiVersionDatabasesInstancesApiVersionEnum::V4,
            ..Default::default()
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
        .delete(linode_api::resources::databases::postgresql::instances::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionDatabasesPostgresqlInstancesInstanceIdApiVersionEnum::V4,
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
        .postgresql()
        .instances()
        .list(linode_api::resources::databases::postgresql::instances::ListRequest {
            api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesApiVersionEnum::V4,
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
        .get(linode_api::resources::databases::postgresql::instances::GetRequest {
            api_version: linode_api::models::GetApiVersionDatabasesPostgresqlInstancesInstanceIdApiVersionEnum::V4,
            instance_id: 123,
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
        .create(linode_api::resources::databases::postgresql::instances::CreateRequest {
            api_version: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionDatabasesPostgresqlInstancesBody {
                allow_list: Some(
                    vec!["203.0.113.1/32".to_string(), "192.0.1.0/24".to_string()],
                ),
                cluster_size: Some(3),
                encrypted: Some(false),
                engine: "postgresql/13.2".to_string(),
                label: "example-db".to_string(),
                region: "us-east".to_string(),
                replication_commit_type: Some(
                    linode_api::models::PostApiVersionDatabasesPostgresqlInstancesBodyReplicationCommitTypeEnum::Local,
                ),
                replication_type: Some(
                    linode_api::models::PostApiVersionDatabasesPostgresqlInstancesBodyReplicationTypeEnum::Asynch,
                ),
                ssl_connection: Some(true),
                type_field: "g6-dedicated-2".to_string(),
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
        .databases()
        .postgresql()
        .instances()
        .put(linode_api::resources::databases::postgresql::instances::PutRequest {
            api_version: linode_api::models::PutApiVersionDatabasesPostgresqlInstancesInstanceIdApiVersionEnum::V4,
            instance_id: 123,
            data: linode_api::models::PutApiVersionDatabasesPostgresqlInstancesInstanceIdBody {
                allow_list: Some(
                    vec!["203.0.113.1/32".to_string(), "192.0.1.0/24".to_string()],
                ),
                label: Some("example-db".to_string()),
                type_field: Some("g6-standard-1".to_string()),
                updates: Some(linode_api::models::PutApiVersionDatabasesPostgresqlInstancesInstanceIdBodyUpdates {
                    day_of_week: Some(1),
                    duration: Some(3),
                    frequency: Some(
                        linode_api::models::PutApiVersionDatabasesPostgresqlInstancesInstanceIdBodyUpdatesFrequencyEnum::Weekly,
                    ),
                    hour_of_day: Some(0),
                    week_of_month: linode_api::Patch::new(123),
                    ..Default::default()
                }),
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
        .delete(linode_api::resources::databases::mysql::instances::DeleteRequest {
            api_version: linode_api::models::DeleteApiVersionDatabasesMysqlInstancesInstanceIdApiVersionEnum::V4,
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
        .list(linode_api::resources::databases::mysql::instances::ListRequest {
            api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesApiVersionEnum::V4,
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
        .get(linode_api::resources::databases::mysql::instances::GetRequest {
            api_version: linode_api::models::GetApiVersionDatabasesMysqlInstancesInstanceIdApiVersionEnum::V4,
            instance_id: 123,
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
        .create(linode_api::resources::databases::mysql::instances::CreateRequest {
            api_version: linode_api::models::PostApiVersionDatabasesMysqlInstancesApiVersionEnum::V4,
            data: linode_api::models::PostApiVersionDatabasesMysqlInstancesBody {
                allow_list: Some(
                    vec!["203.0.113.1/32".to_string(), "192.0.1.0/24".to_string()],
                ),
                cluster_size: Some(3),
                encrypted: Some(false),
                engine: "mysql/8.0.26".to_string(),
                label: "example-db".to_string(),
                region: "us-east".to_string(),
                replication_type: Some(
                    linode_api::models::PostApiVersionDatabasesMysqlInstancesBodyReplicationTypeEnum::SemiSynch,
                ),
                ssl_connection: Some(true),
                type_field: "g6-dedicated-2".to_string(),
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
        .databases()
        .mysql()
        .instances()
        .put(linode_api::resources::databases::mysql::instances::PutRequest {
            api_version: linode_api::models::PutApiVersionDatabasesMysqlInstancesInstanceIdApiVersionEnum::V4,
            instance_id: 123,
            data: linode_api::models::PutApiVersionDatabasesMysqlInstancesInstanceIdBody {
                allow_list: Some(
                    vec!["203.0.113.1/32".to_string(), "192.0.1.0/24".to_string()],
                ),
                label: Some("example-db".to_string()),
                type_field: Some("g6-standard-1".to_string()),
                updates: Some(linode_api::models::PutApiVersionDatabasesMysqlInstancesInstanceIdBodyUpdates {
                    day_of_week: Some(1),
                    duration: Some(3),
                    frequency: Some(
                        linode_api::models::PutApiVersionDatabasesMysqlInstancesInstanceIdBodyUpdatesFrequencyEnum::Weekly,
                    ),
                    hour_of_day: Some(0),
                    week_of_month: linode_api::Patch::new(123),
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
        .await;
    println!("{:?}", res);
    assert!(res.is_ok());
}
