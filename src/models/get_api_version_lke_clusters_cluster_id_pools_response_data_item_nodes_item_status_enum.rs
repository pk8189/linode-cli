#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLkeClustersClusterIdPoolsResponseDataItemNodesItemStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "not_ready"))]
    #[serde(rename = "not_ready")]
    NotReady,
    #[cfg_attr(feature = "cli", value(name = "ready"))]
    #[serde(rename = "ready")]
    Ready,
}
impl std::fmt::Display
for GetApiVersionLkeClustersClusterIdPoolsResponseDataItemNodesItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLkeClustersClusterIdPoolsResponseDataItemNodesItemStatusEnum::NotReady => {
                "not_ready"
            }
            GetApiVersionLkeClustersClusterIdPoolsResponseDataItemNodesItemStatusEnum::Ready => {
                "ready"
            }
        };
        write!(f, "{}", str_val)
    }
}
