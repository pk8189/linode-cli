#[derive(Clone, Debug)]
pub struct ManagedClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ManagedClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn contacts(
        &self,
    ) -> crate::resources::managed::contacts::resource_client::ContactsClient {
        crate::resources::managed::contacts::resource_client::ContactsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn services(
        &self,
    ) -> crate::resources::managed::services::resource_client::ServicesClient {
        crate::resources::managed::services::resource_client::ServicesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn credentials(
        &self,
    ) -> crate::resources::managed::credentials::resource_client::CredentialsClient {
        crate::resources::managed::credentials::resource_client::CredentialsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn issues(
        &self,
    ) -> crate::resources::managed::issues::resource_client::IssuesClient {
        crate::resources::managed::issues::resource_client::IssuesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn linode_settings(
        &self,
    ) -> crate::resources::managed::linode_settings::resource_client::LinodeSettingsClient {
        crate::resources::managed::linode_settings::resource_client::LinodeSettingsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn stats(
        &self,
    ) -> crate::resources::managed::stats::resource_client::StatsClient {
        crate::resources::managed::stats::resource_client::StatsClient::new(
            self.base_client.clone(),
        )
    }
}
