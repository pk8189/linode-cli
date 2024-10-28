#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionLkeClustersClusterIdPoolsBodyDisksItemTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ext4"))]
    #[serde(rename = "ext4")]
    Ext4,
    #[cfg_attr(feature = "cli", value(name = "raw"))]
    #[serde(rename = "raw")]
    Raw,
}
impl std::fmt::Display for PostApiVersionLkeClustersClusterIdPoolsBodyDisksItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionLkeClustersClusterIdPoolsBodyDisksItemTypeEnum::Ext4 => "ext4",
            PostApiVersionLkeClustersClusterIdPoolsBodyDisksItemTypeEnum::Raw => "raw",
        };
        write!(f, "{}", str_val)
    }
}
