#[derive(Clone, Debug)]
pub struct LkeClient {
    base_client: crate::core::base_client::BaseClient,
}
impl LkeClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn clusters(
        &self,
    ) -> crate::resources::lke::clusters::resource_client::ClustersClient {
        crate::resources::lke::clusters::resource_client::ClustersClient::new(
            self.base_client.clone(),
        )
    }
    pub fn types_resource(
        &self,
    ) -> crate::resources::lke::types_resource::resource_client::TypesClient {
        crate::resources::lke::types_resource::resource_client::TypesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn versions(
        &self,
    ) -> crate::resources::lke::versions::resource_client::VersionsClient {
        crate::resources::lke::versions::resource_client::VersionsClient::new(
            self.base_client.clone(),
        )
    }
}
