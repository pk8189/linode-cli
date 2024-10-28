#[derive(Clone, Debug)]
pub struct DatabasesClient {
    base_client: crate::core::base_client::BaseClient,
}
impl DatabasesClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn mysql(
        &self,
    ) -> crate::resources::databases::mysql::resource_client::MysqlClient {
        crate::resources::databases::mysql::resource_client::MysqlClient::new(
            self.base_client.clone(),
        )
    }
    pub fn postgresql(
        &self,
    ) -> crate::resources::databases::postgresql::resource_client::PostgresqlClient {
        crate::resources::databases::postgresql::resource_client::PostgresqlClient::new(
            self.base_client.clone(),
        )
    }
    pub fn engines(
        &self,
    ) -> crate::resources::databases::engines::resource_client::EnginesClient {
        crate::resources::databases::engines::resource_client::EnginesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn instances(
        &self,
    ) -> crate::resources::databases::instances::resource_client::InstancesClient {
        crate::resources::databases::instances::resource_client::InstancesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn types_resource(
        &self,
    ) -> crate::resources::databases::types_resource::resource_client::TypesClient {
        crate::resources::databases::types_resource::resource_client::TypesClient::new(
            self.base_client.clone(),
        )
    }
}
