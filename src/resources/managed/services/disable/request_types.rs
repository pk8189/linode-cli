#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionManagedServicesServiceIdDisableApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "service-id", long = "service-id"))]
    pub service_id: i64,
}
