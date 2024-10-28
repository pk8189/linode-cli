#[derive(Clone, Debug)]
pub struct LinodeClient {
    base_client: crate::core::base_client::BaseClient,
}
impl LinodeClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn instances(
        &self,
    ) -> crate::resources::linode::instances::resource_client::InstancesClient {
        crate::resources::linode::instances::resource_client::InstancesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn stackscripts(
        &self,
    ) -> crate::resources::linode::stackscripts::resource_client::StackscriptsClient {
        crate::resources::linode::stackscripts::resource_client::StackscriptsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn kernels(
        &self,
    ) -> crate::resources::linode::kernels::resource_client::KernelsClient {
        crate::resources::linode::kernels::resource_client::KernelsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn types_resource(
        &self,
    ) -> crate::resources::linode::types_resource::resource_client::TypesClient {
        crate::resources::linode::types_resource::resource_client::TypesClient::new(
            self.base_client.clone(),
        )
    }
}
