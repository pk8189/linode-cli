#[derive(Clone, Debug)]
pub struct RebuildClient {
    base_client: crate::core::base_client::BaseClient,
}
impl RebuildClient {
    pub(crate) fn new(base_client: crate::core::base_client::BaseClient) -> Self {
        Self { base_client }
    }
    /// Rebuilds a NodeBalancer Config and its Nodes that you have permission to modify.
    ///
    /// Use this operation to update a NodeBalancer's Config and Nodes with a single request.
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
    ///     linode-cli nodebalancers config-rebuild \
    ///   12345 4567 \
    ///   --port 443 \
    ///   --protocol https \
    ///   --algorithm roundrobin \
    ///   --stickiness http_cookie \
    ///   --check http_body \
    ///   --check_interval 90 \
    ///   --check_timeout 10 \
    ///   --check_attempts 3 \
    ///   --check_path "/test" \
    ///   --check_body "it works" \
    ///   --check_passive true \
    ///   --proxy_protocol "none" \
    ///   --ssl_cert "-----BEGIN CERTIFICATE-----
    ///               CERTIFICATE_INFORMATION
    ///               -----END CERTIFICATE-----" \
    ///   --ssl_key "-----BEGIN PRIVATE KEY-----
    ///              PRIVATE_KEY_INFORMATION
    ///              -----END PRIVATE KEY-----" \
    ///   --cipher_suite recommended \
    ///   --nodes '{"address":"192.168.210.120:80","label":"node1","weight":50,"mode":"accept"}' \
    ///   --nodes '{"address":"192.168.210.122:80","label":"node2","weight":50,"mode":"accept"}'
    ///     `-`-`
    ///
    ///     [Learn more...](https://www.linode.com/docs/products/tools/cli/get-started/)
    ///
    /// - __OAuth scopes__.
    ///
    ///     `-`-`
    ///     nodebalancers:read_write
    ///     `-`-`
    ///
    ///     [Learn more...](https://techdocs.akamai.com/linode-api/reference/get-started#oauth)
    pub async fn create(
        &self,
        request: super::request_types::CreateRequest,
    ) -> crate::SdkResult<
        crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponse,
    > {
        let url = self
            .base_client
            .build_url(
                &format!(
                    "/{}/nodebalancers/{}/configs/{}/rebuild", & request.api_version, &
                    request.node_balancer_id, & request.config_id
                ),
            );
        let mut builder = reqwest::Client::default().post(&url);
        builder = builder.header("x-sideko-sdk-language", "rust");
        builder = builder.header("content-type", "application/json");
        builder = builder.json(&request.data);
        builder = self
            .base_client
            .apply_auths_to_builder(builder, &["oauth", "personalAccessToken"]);
        let mut response = builder.send().await?;
        response = self.base_client.error_for_status(response).await?;
        crate::core::response::process_json::<
            crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponse,
        >(response)
            .await
    }
}
