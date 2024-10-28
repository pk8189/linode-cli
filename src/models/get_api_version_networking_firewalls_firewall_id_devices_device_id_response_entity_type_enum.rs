#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdResponseEntityTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "linode"))]
    #[serde(rename = "linode")]
    Linode,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer"))]
    #[serde(rename = "nodebalancer")]
    Nodebalancer,
}
impl std::fmt::Display
for GetApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdResponseEntityTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdResponseEntityTypeEnum::Linode => {
                "linode"
            }
            GetApiVersionNetworkingFirewallsFirewallIdDevicesDeviceIdResponseEntityTypeEnum::Nodebalancer => {
                "nodebalancer"
            }
        };
        write!(f, "{}", str_val)
    }
}
