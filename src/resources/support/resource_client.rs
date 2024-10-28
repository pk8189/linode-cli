#[derive(Clone, Debug)]
pub struct SupportClient {
    base_client: crate::core::base_client::BaseClient,
}
impl SupportClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn tickets(
        &self,
    ) -> crate::resources::support::tickets::resource_client::TicketsClient {
        crate::resources::support::tickets::resource_client::TicketsClient::new(
            self.base_client.clone(),
        )
    }
}
