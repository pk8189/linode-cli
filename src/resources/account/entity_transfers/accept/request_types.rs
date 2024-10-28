#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionAccountEntityTransfersTokenAcceptApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "token", long = "token"))]
    pub token: String,
}
