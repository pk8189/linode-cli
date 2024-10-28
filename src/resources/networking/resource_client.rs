#[derive(Clone, Debug)]
pub struct NetworkingClient {
    base_client: crate::core::base_client::BaseClient,
}
impl NetworkingClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn firewalls(
        &self,
    ) -> crate::resources::networking::firewalls::resource_client::FirewallsClient {
        crate::resources::networking::firewalls::resource_client::FirewallsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn ipv6(
        &self,
    ) -> crate::resources::networking::ipv6::resource_client::Ipv6Client {
        crate::resources::networking::ipv6::resource_client::Ipv6Client::new(
            self.base_client.clone(),
        )
    }
    pub fn ips(&self) -> crate::resources::networking::ips::resource_client::IpsClient {
        crate::resources::networking::ips::resource_client::IpsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn vlans(
        &self,
    ) -> crate::resources::networking::vlans::resource_client::VlansClient {
        crate::resources::networking::vlans::resource_client::VlansClient::new(
            self.base_client.clone(),
        )
    }
    pub fn ipv4(
        &self,
    ) -> crate::resources::networking::ipv4::resource_client::Ipv4Client {
        crate::resources::networking::ipv4::resource_client::Ipv4Client::new(
            self.base_client.clone(),
        )
    }
}
