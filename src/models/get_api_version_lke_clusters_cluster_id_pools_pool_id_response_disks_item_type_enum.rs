#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseDisksItemTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ext4"))]
    #[serde(rename = "ext4")]
    Ext4,
    #[cfg_attr(feature = "cli", value(name = "raw"))]
    #[serde(rename = "raw")]
    Raw,
}
impl std::fmt::Display
for GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseDisksItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseDisksItemTypeEnum::Ext4 => {
                "ext4"
            }
            GetApiVersionLkeClustersClusterIdPoolsPoolIdResponseDisksItemTypeEnum::Raw => {
                "raw"
            }
        };
        write!(f, "{}", str_val)
    }
}
