#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionProfileGrantsResponseNodebalancerItemPermissionsEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "read_only"))]
    #[serde(rename = "read_only")]
    ReadOnly,
    #[cfg_attr(feature = "cli", value(name = "read_write"))]
    #[serde(rename = "read_write")]
    ReadWrite,
}
impl std::fmt::Display
for GetApiVersionProfileGrantsResponseNodebalancerItemPermissionsEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionProfileGrantsResponseNodebalancerItemPermissionsEnum::ReadOnly => {
                "read_only"
            }
            GetApiVersionProfileGrantsResponseNodebalancerItemPermissionsEnum::ReadWrite => {
                "read_write"
            }
        };
        write!(f, "{}", str_val)
    }
}
