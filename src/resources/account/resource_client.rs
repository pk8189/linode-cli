#[derive(Clone, Debug)]
pub struct AccountClient {
    base_client: crate::core::base_client::BaseClient,
}
impl AccountClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    pub fn entity_transfers(
        &self,
    ) -> crate::resources::account::entity_transfers::resource_client::EntityTransfersClient {
        crate::resources::account::entity_transfers::resource_client::EntityTransfersClient::new(
            self.base_client.clone(),
        )
    }
    pub fn oauth_clients(
        &self,
    ) -> crate::resources::account::oauth_clients::resource_client::OauthClientsClient {
        crate::resources::account::oauth_clients::resource_client::OauthClientsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn payment_methods(
        &self,
    ) -> crate::resources::account::payment_methods::resource_client::PaymentMethodsClient {
        crate::resources::account::payment_methods::resource_client::PaymentMethodsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn service_transfers(
        &self,
    ) -> crate::resources::account::service_transfers::resource_client::ServiceTransfersClient {
        crate::resources::account::service_transfers::resource_client::ServiceTransfersClient::new(
            self.base_client.clone(),
        )
    }
    pub fn users(
        &self,
    ) -> crate::resources::account::users::resource_client::UsersClient {
        crate::resources::account::users::resource_client::UsersClient::new(
            self.base_client.clone(),
        )
    }
    pub fn agreements(
        &self,
    ) -> crate::resources::account::agreements::resource_client::AgreementsClient {
        crate::resources::account::agreements::resource_client::AgreementsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn availability(
        &self,
    ) -> crate::resources::account::availability::resource_client::AvailabilityClient {
        crate::resources::account::availability::resource_client::AvailabilityClient::new(
            self.base_client.clone(),
        )
    }
    pub fn betas(
        &self,
    ) -> crate::resources::account::betas::resource_client::BetasClient {
        crate::resources::account::betas::resource_client::BetasClient::new(
            self.base_client.clone(),
        )
    }
    pub fn child_accounts(
        &self,
    ) -> crate::resources::account::child_accounts::resource_client::ChildAccountsClient {
        crate::resources::account::child_accounts::resource_client::ChildAccountsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn events(
        &self,
    ) -> crate::resources::account::events::resource_client::EventsClient {
        crate::resources::account::events::resource_client::EventsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn invoices(
        &self,
    ) -> crate::resources::account::invoices::resource_client::InvoicesClient {
        crate::resources::account::invoices::resource_client::InvoicesClient::new(
            self.base_client.clone(),
        )
    }
    pub fn logins(
        &self,
    ) -> crate::resources::account::logins::resource_client::LoginsClient {
        crate::resources::account::logins::resource_client::LoginsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn maintenance(
        &self,
    ) -> crate::resources::account::maintenance::resource_client::MaintenanceClient {
        crate::resources::account::maintenance::resource_client::MaintenanceClient::new(
            self.base_client.clone(),
        )
    }
    pub fn notifications(
        &self,
    ) -> crate::resources::account::notifications::resource_client::NotificationsClient {
        crate::resources::account::notifications::resource_client::NotificationsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn payments(
        &self,
    ) -> crate::resources::account::payments::resource_client::PaymentsClient {
        crate::resources::account::payments::resource_client::PaymentsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn settings(
        &self,
    ) -> crate::resources::account::settings::resource_client::SettingsClient {
        crate::resources::account::settings::resource_client::SettingsClient::new(
            self.base_client.clone(),
        )
    }
    pub fn transfer(
        &self,
    ) -> crate::resources::account::transfer::resource_client::TransferClient {
        crate::resources::account::transfer::resource_client::TransferClient::new(
            self.base_client.clone(),
        )
    }
    pub fn cancel(
        &self,
    ) -> crate::resources::account::cancel::resource_client::CancelClient {
        crate::resources::account::cancel::resource_client::CancelClient::new(
            self.base_client.clone(),
        )
    }
    pub fn credit_card(
        &self,
    ) -> crate::resources::account::credit_card::resource_client::CreditCardClient {
        crate::resources::account::credit_card::resource_client::CreditCardClient::new(
            self.base_client.clone(),
        )
    }
    pub fn promo_codes(
        &self,
    ) -> crate::resources::account::promo_codes::resource_client::PromoCodesClient {
        crate::resources::account::promo_codes::resource_client::PromoCodesClient::new(
            self.base_client.clone(),
        )
    }
    /// Returns the contact and billing information related to your Account.
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
    ///     linode-cli account view
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     account:read_only
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn list(
        &self,
        request: super::request_types::ListRequest,
    ) -> crate::SdkResult<crate::models::GetApiVersionAccountResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/account", & request.api_version));
        let mut builder = reqwest::Client::default().get(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::GetApiVersionAccountResponse,
        >(response)
            .await
    }
    /// Updates contact and billing information related to your account. If you exclude any properties from the request, the operation leaves them unchanged.
    ///
    /// __Note__. When updating an account's `country` to `US`, you'll get an error if the account's `zip` is not a valid US zip code.
    ///
    /// __Parent and child accounts__
    ///
    /// In a [parent and child account](https://www.linode.com/docs/guides/parent-child-accounts/) environment, the following apply:
    ///
    /// - You can't change the `company` for a parent account. Akamai uses this value to set the name for a child account parent user (proxy user) on any child account.
    ///
    /// - Child account users can't run this operation. These users don't have access to billing-related operations.
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
    ///     linode-cli account update \
    ///   --address_1 "123 Main St." \
    ///   --address_2 "Suite 101" \
    ///   --city Philadelphia \
    ///   --company My Company \ LLC \
    ///   --country US \
    ///   --email jsmith@mycompany.com \
    ///   --first_name John \
    ///   --last_name Smith \
    ///   --phone 555-555-1212 \
    ///   --state PA \
    ///   --tax_id ATU99999999 \
    ///   --zip 19102
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
    ) -> crate::SdkResult<crate::models::PutApiVersionAccountResponse> {
        let url = self
            .base_client
            .build_url(&format!("/{}/account", & request.api_version));
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
            crate::models::PutApiVersionAccountResponse,
        >(response)
            .await
    }
}
