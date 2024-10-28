#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct DeleteRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::DeleteApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "firewall-id", long = "firewall-id"))]
    pub firewall_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "device-id", long = "device-id"))]
    pub device_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct ListRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::GetApiVersionNetworkingFirewallsFirewallIdDevicesApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "firewall-id", long = "firewall-id"))]
    pub firewall_id: i64,
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
    pub api_version: crate::models::GetApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "firewall-id", long = "firewall-id"))]
    pub firewall_id: i64,
    #[cfg_attr(feature = "cli", arg(id = "device-id", long = "device-id"))]
    pub device_id: i64,
}
#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionNetworkingFirewallsFirewallIdDevicesApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "firewall-id", long = "firewall-id"))]
    pub firewall_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionNetworkingFirewallsFirewallIdDevicesBody,
}
