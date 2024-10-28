#[derive(Clone, Debug)]
pub struct NetworkTransferClient {
    base_client: crate::core::base_client::BaseClient,
}
impl NetworkTransferClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn prices(
        &self,
    ) -> crate::resources::network_transfer::prices::resource_client::PricesClient {
        crate::resources::network_transfer::prices::resource_client::PricesClient::new(
            self.base_client.clone(),
        )
    }
}
