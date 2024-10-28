#[derive(Clone, Debug, Default)]
pub struct Client {
    base_client: crate::core::base_client::BaseClient,
}
impl Client {
    /// Override the default URL environment
    pub fn with_environment(mut self, env: crate::environment::Environment) -> Self {
        self.base_client.base_url = crate::environment::BaseUrl::Env(env);
        self
    }
    /// Override the default URL with a custom base URL
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_client.base_url = crate::environment::BaseUrl::Custom(base_url.into());
        self
    }
    /// Override the default underlying reqwest client
    pub fn with_client(mut self, client: reqwest::Client) -> Self {
        self.base_client.client = client;
        self
    }
    pub fn with_oauth(mut self, val: &str) -> Self {
        self.base_client
            .auth
            .insert("oauth".into(), crate::core::auth::AuthProvider::Bearer(val.into()));
        self
    }
    pub fn with_personal_access_token(mut self, val: &str) -> Self {
        self.base_client
            .auth
            .insert(
                "personalAccessToken".into(),
                crate::core::auth::AuthProvider::Bearer(val.into()),
            );
        self
    }
    pub fn account(&self) -> crate::resources::account::resource_client::AccountClient {
        crate::resources::account::resource_client::AccountClient::new(
            self.base_client.clone(),
        )
    }
    pub fn databases(
        &self,
    ) -> crate::resources::databases::resource_client::DatabasesClient {
        crate::resources::databases::resource_client::DatabasesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn domains(&self) -> crate::resources::domains::resource_client::DomainsClient {
        crate::resources::domains::resource_client::DomainsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn images(&self) -> crate::resources::images::resource_client::ImagesClient {
        crate::resources::images::resource_client::ImagesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn linode(&self) -> crate::resources::linode::resource_client::LinodeClient {
        crate::resources::linode::resource_client::LinodeClient::new(
            self.base_client.clone(),
        )
    }
    pub fn lke(&self) -> crate::resources::lke::resource_client::LkeClient {
        crate::resources::lke::resource_client::LkeClient::new(self.base_client.clone())
    }
    pub fn longview(
        &self,
    ) -> crate::resources::longview::resource_client::LongviewClient {
        crate::resources::longview::resource_client::LongviewClient::new(
            self.base_client.clone(),
        )
    }
    pub fn managed(&self) -> crate::resources::managed::resource_client::ManagedClient {
        crate::resources::managed::resource_client::ManagedClient::new(
            self.base_client.clone(),
        )
    }
    pub fn networking(
        &self,
    ) -> crate::resources::networking::resource_client::NetworkingClient {
        crate::resources::networking::resource_client::NetworkingClient::new(
            self.base_client.clone(),
        )
    }
    pub fn nodebalancers(
        &self,
    ) -> crate::resources::nodebalancers::resource_client::NodebalancersClient {
        crate::resources::nodebalancers::resource_client::NodebalancersClient::new(
            self.base_client.clone(),
        )
    }
    pub fn object_storage(
        &self,
    ) -> crate::resources::object_storage::resource_client::ObjectStorageClient {
        crate::resources::object_storage::resource_client::ObjectStorageClient::new(
            self.base_client.clone(),
        )
    }
    pub fn placement(
        &self,
    ) -> crate::resources::placement::resource_client::PlacementClient {
        crate::resources::placement::resource_client::PlacementClient::new(
            self.base_client.clone(),
        )
    }
    pub fn profile_resource(
        &self,
    ) -> crate::resources::profile_resource::resource_client::ProfileClient {
        crate::resources::profile_resource::resource_client::ProfileClient::new(
            self.base_client.clone(),
        )
    }
    pub fn tags(&self) -> crate::resources::tags::resource_client::TagsClient {
        crate::resources::tags::resource_client::TagsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn volumes(&self) -> crate::resources::volumes::resource_client::VolumesClient {
        crate::resources::volumes::resource_client::VolumesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn vpcs(&self) -> crate::resources::vpcs::resource_client::VpcsClient {
        crate::resources::vpcs::resource_client::VpcsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn betas(&self) -> crate::resources::betas::resource_client::BetasClient {
        crate::resources::betas::resource_client::BetasClient::new(
            self.base_client.clone(),
        )
    }
    pub fn network_transfer(
        &self,
    ) -> crate::resources::network_transfer::resource_client::NetworkTransferClient {
        crate::resources::network_transfer::resource_client::NetworkTransferClient::new(
            self.base_client.clone(),
        )
    }
    pub fn regions(&self) -> crate::resources::regions::resource_client::RegionsClient {
        crate::resources::regions::resource_client::RegionsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn support(&self) -> crate::resources::support::resource_client::SupportClient {
        crate::resources::support::resource_client::SupportClient::new(
            self.base_client.clone(),
        )
    }
}
