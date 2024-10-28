#[derive(Clone, Debug)]
pub struct ObjectStorageClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ObjectStorageClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn buckets(
        &self,
    ) -> crate::resources::object_storage::buckets::resource_client::BucketsClient {
        crate::resources::object_storage::buckets::resource_client::BucketsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn keys(
        &self,
    ) -> crate::resources::object_storage::keys::resource_client::KeysClient {
        crate::resources::object_storage::keys::resource_client::KeysClient::new(
            self.base_client.clone(),
        )
    }
    pub fn clusters(
        &self,
    ) -> crate::resources::object_storage::clusters::resource_client::ClustersClient {
        crate::resources::object_storage::clusters::resource_client::ClustersClient::new(
            self.base_client.clone(),
        )
    }
    pub fn transfer(
        &self,
    ) -> crate::resources::object_storage::transfer::resource_client::TransferClient {
        crate::resources::object_storage::transfer::resource_client::TransferClient::new(
            self.base_client.clone(),
        )
    }
    pub fn types_resource(
        &self,
    ) -> crate::resources::object_storage::types_resource::resource_client::TypesClient {
        crate::resources::object_storage::types_resource::resource_client::TypesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn cancel(
        &self,
    ) -> crate::resources::object_storage::cancel::resource_client::CancelClient {
        crate::resources::object_storage::cancel::resource_client::CancelClient::new(
            self.base_client.clone(),
        )
    }
}
