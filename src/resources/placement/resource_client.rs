#[derive(Clone, Debug)]
pub struct PlacementClient {
    base_client: crate::core::base_client::BaseClient,
}
impl PlacementClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn groups(
        &self,
    ) -> crate::resources::placement::groups::resource_client::GroupsClient {
        crate::resources::placement::groups::resource_client::GroupsClient::new(
            self.base_client.clone(),
        )
    }
}
