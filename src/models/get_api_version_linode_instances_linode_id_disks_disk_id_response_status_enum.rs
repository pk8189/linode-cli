#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdDisksDiskIdResponseStatusEnum {
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
for GetApiVersionLinodeInstancesLinodeIdDisksDiskIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdDisksDiskIdResponseStatusEnum::Deleting => {
                "deleting"
            }
            GetApiVersionLinodeInstancesLinodeIdDisksDiskIdResponseStatusEnum::NotReady => {
                "not ready"
            }
            GetApiVersionLinodeInstancesLinodeIdDisksDiskIdResponseStatusEnum::Ready => {
                "ready"
            }
        };
        write!(f, "{}", str_val)
    }
}
