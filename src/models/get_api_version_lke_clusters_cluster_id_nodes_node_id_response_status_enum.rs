#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLkeClustersClusterIdNodesNodeIdResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "not_ready"))]
    #[serde(rename = "not_ready")]
    NotReady,
    #[cfg_attr(feature = "cli", value(name = "ready"))]
    #[serde(rename = "ready")]
    Ready,
}
impl std::fmt::Display
for GetApiVersionLkeClustersClusterIdNodesNodeIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLkeClustersClusterIdNodesNodeIdResponseStatusEnum::NotReady => {
                "not_ready"
            }
            GetApiVersionLkeClustersClusterIdNodesNodeIdResponseStatusEnum::Ready => {
                "ready"
            }
        };
        write!(f, "{}", str_val)
    }
}
