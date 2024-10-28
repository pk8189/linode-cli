#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLkeClustersClusterIdPoolsResponseDataItemDiskEncryptionEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "enabled"))]
    #[serde(rename = "enabled")]
    Enabled,
}
impl std::fmt::Display
for GetApiVersionLkeClustersClusterIdPoolsResponseDataItemDiskEncryptionEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLkeClustersClusterIdPoolsResponseDataItemDiskEncryptionEnum::Enabled => {
                "enabled"
            }
        };
        write!(f, "{}", str_val)
    }
}
