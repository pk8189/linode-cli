#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionManagedCredentialsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "page", long = "page"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "page-size", long = "page-size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionManagedCredentialsCredentialIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "credential-id", long = "credential-id"))]
    pub credential_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionManagedCredentialsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "data", long = "data"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<serde_json::Value>)
    )]
    pub data: serde_json::Value,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionManagedCredentialsCredentialIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "credential-id", long = "credential-id"))]
    pub credential_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionManagedCredentialsCredentialIdBody,
}
