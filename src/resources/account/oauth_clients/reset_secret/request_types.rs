#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionAccountOauthClientsClientIdResetSecretApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "client-id", long = "client-id"))]
    pub client_id: String,
}
