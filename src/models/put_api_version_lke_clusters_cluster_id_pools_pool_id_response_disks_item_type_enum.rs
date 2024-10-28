#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionLkeClustersClusterIdPoolsPoolIdResponseDisksItemTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ext4"))]
    #[serde(rename = "ext4")]
    Ext4,
    #[cfg_attr(feature = "cli", value(name = "raw"))]
    #[serde(rename = "raw")]
    Raw,
}
impl std::fmt::Display
for PutApiVersionLkeClustersClusterIdPoolsPoolIdResponseDisksItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionLkeClustersClusterIdPoolsPoolIdResponseDisksItemTypeEnum::Ext4 => {
                "ext4"
            }
            PutApiVersionLkeClustersClusterIdPoolsPoolIdResponseDisksItemTypeEnum::Raw => {
                "raw"
            }
        };
        write!(f, "{}", str_val)
    }
}
