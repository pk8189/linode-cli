#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionAccountOauthClientsClientIdThumbnailApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "client-id", long = "client-id"))]
    pub client_id: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionAccountOauthClientsClientIdThumbnailApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "client-id", long = "client-id"))]
    pub client_id: String,
    #[cfg_attr(feature = "cli", arg(id = "data", long = "data"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_upload_file)
    )]
    pub data: crate::UploadFile,
}
