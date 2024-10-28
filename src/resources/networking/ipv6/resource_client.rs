#[derive(Clone, Debug)]
pub struct Ipv6Client {
    base_client: crate::core::base_client::BaseClient,
}
impl Ipv6Client {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn ranges(
        &self,
    ) -> crate::resources::networking::ipv6::ranges::resource_client::RangesClient {
        crate::resources::networking::ipv6::ranges::resource_client::RangesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn pools(
        &self,
    ) -> crate::resources::networking::ipv6::pools::resource_client::PoolsClient {
        crate::resources::networking::ipv6::pools::resource_client::PoolsClient::new(
            self.base_client.clone(),
        )
    }
}
