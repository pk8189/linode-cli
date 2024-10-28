#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::DeleteApiVersionManagedServicesServiceIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "service-id", long = "service-id"))]
    pub service_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionManagedServicesApiVersionEnum,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionManagedServicesServiceIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "service-id", long = "service-id"))]
    pub service_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionManagedServicesApiVersionEnum,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionManagedServicesBody,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PutApiVersionManagedServicesServiceIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "service-id", long = "service-id"))]
    pub service_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PutApiVersionManagedServicesServiceIdBody,
}
