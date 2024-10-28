#[derive(Clone, Debug)]
pub struct MysqlClient {
    base_client: crate::core::base_client::BaseClient,
}
impl MysqlClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn instances(
        &self,
    ) -> crate::resources::databases::mysql::instances::resource_client::InstancesClient {
        crate::resources::databases::mysql::instances::resource_client::InstancesClient::new(
            self.base_client.clone(),
        )
    }
}
