#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionAccountUsersUsernameGrantsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "username", long = "username"))]
    pub username: String,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionAccountUsersUsernameGrantsApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "username", long = "username"))]
    pub username: String,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionAccountUsersUsernameGrantsBody,
}
