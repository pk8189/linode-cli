#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4ReservedItemTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ipv4"))]
    #[serde(rename = "ipv4")]
    Ipv4,
    #[cfg_attr(feature = "cli", value(name = "ipv6"))]
    #[serde(rename = "ipv6")]
    Ipv6,
    #[cfg_attr(feature = "cli", value(name = "ipv6/pool"))]
    #[serde(rename = "ipv6/pool")]
    Ipv6Pool,
    #[cfg_attr(feature = "cli", value(name = "ipv6/range"))]
    #[serde(rename = "ipv6/range")]
    Ipv6Range,
}
impl std::fmt::Display
for GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4ReservedItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4ReservedItemTypeEnum::Ipv4 => {
                "ipv4"
            }
            GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4ReservedItemTypeEnum::Ipv6 => {
                "ipv6"
            }
            GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4ReservedItemTypeEnum::Ipv6Pool => {
                "ipv6/pool"
            }
            GetApiVersionLinodeInstancesLinodeIdIpsResponseIpv4ReservedItemTypeEnum::Ipv6Range => {
                "ipv6/range"
            }
        };
        write!(f, "{}", str_val)
    }
}
