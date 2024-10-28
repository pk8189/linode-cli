#[derive(Clone, Debug)]
pub struct ProfileClient {
    base_client: crate::core::base_client::BaseClient,
}
impl ProfileClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn apps(
        &self,
    ) -> crate::resources::profile_resource::apps::resource_client::AppsClient {
        crate::resources::profile_resource::apps::resource_client::AppsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn devices(
        &self,
    ) -> crate::resources::profile_resource::devices::resource_client::DevicesClient {
        crate::resources::profile_resource::devices::resource_client::DevicesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn phone_number(
        &self,
    ) -> crate::resources::profile_resource::phone_number::resource_client::PhoneNumberClient {
        crate::resources::profile_resource::phone_number::resource_client::PhoneNumberClient::new(
            self.base_client.clone(),
        )
    }
    pub fn sshkeys(
        &self,
    ) -> crate::resources::profile_resource::sshkeys::resource_client::SshkeysClient {
        crate::resources::profile_resource::sshkeys::resource_client::SshkeysClient::new(
            self.base_client.clone(),
        )
    }
    pub fn tokens(
        &self,
    ) -> crate::resources::profile_resource::tokens::resource_client::TokensClient {
        crate::resources::profile_resource::tokens::resource_client::TokensClient::new(
            self.base_client.clone(),
        )
    }
    pub fn grants(
        &self,
    ) -> crate::resources::profile_resource::grants::resource_client::GrantsClient {
        crate::resources::profile_resource::grants::resource_client::GrantsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn logins(
        &self,
    ) -> crate::resources::profile_resource::logins::resource_client::LoginsClient {
        crate::resources::profile_resource::logins::resource_client::LoginsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn preferences(
        &self,
    ) -> crate::resources::profile_resource::preferences::resource_client::PreferencesClient {
        crate::resources::profile_resource::preferences::resource_client::PreferencesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn security_questions(
        &self,
    ) -> crate::resources::profile_resource::security_questions::resource_client::SecurityQuestionsClient {
        crate::resources::profile_resource::security_questions::resource_client::SecurityQuestionsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn tfa_disable(
        &self,
    ) -> crate::resources::profile_resource::tfa_disable::resource_client::TfaDisableClient {
        crate::resources::profile_resource::tfa_disable::resource_client::TfaDisableClient::new(
            self.base_client.clone(),
        )
    }
    pub fn tfa_enable(
        &self,
    ) -> crate::resources::profile_resource::tfa_enable::resource_client::TfaEnableClient {
        crate::resources::profile_resource::tfa_enable::resource_client::TfaEnableClient::new(
            self.base_client.clone(),
        )
    }
    pub fn tfa_enable_confirm(
        &self,
    ) -> crate::resources::profile_resource::tfa_enable_confirm::resource_client::TfaEnableConfirmClient {
        crate::resources::profile_resource::tfa_enable_confirm::resource_client::TfaEnableConfirmClient::new(
            self.base_client.clone(),
        )
    }
    /// Returns information about the current User. This can be used to see who is acting in applications where more than one token is managed. For example, in third-party OAuth applications.
    ///
    /// This operation is always accessible, no matter what OAuth scopes the acting token has.
    ///
    ///
    /// <<LB>>
    ///
    /// ---
    ///
    ///
    /// - __CLI__.
    ///
    ///     `-`-`
    ///     linode-cli profile view
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionProfileResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/profile", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionProfileResponse,
        >(response)
            .await
    }
    /// Update information in your Profile.  This operation requires the `account:read_write` OAuth Scope.
    ///
    /// __Parent and child accounts__
    ///
    /// In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:
    ///
    /// - You can't edit the `email` for a child account parent user (proxy user). This value is fixed and set when you provision this environment.
    ///
    ///
    /// <<LB>>
    ///
    /// ---
    ///
    ///
    /// - __CLI__.
    ///
    ///     `-`-`
    ///     linode-cli profile update \
    ///   --email example-user@gmail.com \
    ///   --timezone US/Eastern \
    ///   --email_notifications true \
    ///   --list_auth_method keys_only \
    ///   --two_factor_auth true \
    ///   --restricted false
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     account:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn put(
        &self,
        request: super::request_types::PutRequest,
    ) -> crate::SdkResult<crate::models::PutApiVersionProfileResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/profile", & request.api_version));
        let mut builder = reqwest::Client::default().put(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PutApiVersionProfileResponse,
        >(response)
            .await
    }
}
