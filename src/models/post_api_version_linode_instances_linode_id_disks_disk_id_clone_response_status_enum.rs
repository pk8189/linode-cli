#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionLinodeInstancesLinodeIdDisksDiskIdCloneResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "deleting"))]
    #[serde(rename = "deleting")]
    Deleting,
    #[cfg_attr(feature = "cli", value(name = "not ready"))]
    #[serde(rename = "not ready")]
    NotReady,
    #[cfg_attr(feature = "cli", value(name = "ready"))]
    #[serde(rename = "ready")]
    Ready,
}
impl std::fmt::Display
for PostApiVersionLinodeInstancesLinodeIdDisksDiskIdCloneResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionLinodeInstancesLinodeIdDisksDiskIdCloneResponseStatusEnum::Deleting => {
                "deleting"
            }
            PostApiVersionLinodeInstancesLinodeIdDisksDiskIdCloneResponseStatusEnum::NotReady => {
                "not ready"
            }
            PostApiVersionLinodeInstancesLinodeIdDisksDiskIdCloneResponseStatusEnum::Ready => {
                "ready"
            }
        };
        write!(f, "{}", str_val)
    }
}
