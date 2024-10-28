#[derive(Clone, Debug)]
pub struct LongviewClient {
    base_client: crate::core::base_client::BaseClient,
}
impl LongviewClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn clients(
        &self,
    ) -> crate::resources::longview::clients::resource_client::ClientsClient {
        crate::resources::longview::clients::resource_client::ClientsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn plan(&self) -> crate::resources::longview::plan::resource_client::PlanClient {
        crate::resources::longview::plan::resource_client::PlanClient::new(
            self.base_client.clone(),
        )
    }
    pub fn subscriptions(
        &self,
    ) -> crate::resources::longview::subscriptions::resource_client::SubscriptionsClient {
        crate::resources::longview::subscriptions::resource_client::SubscriptionsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn types_resource(
        &self,
    ) -> crate::resources::longview::types_resource::resource_client::TypesClient {
        crate::resources::longview::types_resource::resource_client::TypesClient::new(
            self.base_client.clone(),
        )
    }
}
