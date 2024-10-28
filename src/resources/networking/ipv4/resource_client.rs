#[derive(Clone, Debug)]
pub struct Ipv4Client {
    base_client: crate::core::base_client::BaseClient,
}
impl Ipv4Client {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn assign(
        &self,
    ) -> crate::resources::networking::ipv4::assign::resource_client::AssignClient {
        crate::resources::networking::ipv4::assign::resource_client::AssignClient::new(
            self.base_client.clone(),
        )
    }
    pub fn share(
        &self,
    ) -> crate::resources::networking::ipv4::share::resource_client::ShareClient {
        crate::resources::networking::ipv4::share::resource_client::ShareClient::new(
            self.base_client.clone(),
        )
    }
}
