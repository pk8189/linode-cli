#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdVolumesResponseDataItemStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "active"))]
    #[serde(rename = "active")]
    Active,
    #[cfg_attr(feature = "cli", value(name = "creating"))]
    #[serde(rename = "creating")]
    Creating,
    #[cfg_attr(feature = "cli", value(name = "key_rotating"))]
    #[serde(rename = "key_rotating")]
    KeyRotating,
    #[cfg_attr(feature = "cli", value(name = "resizing"))]
    #[serde(rename = "resizing")]
    Resizing,
}
impl std::fmt::Display
for GetApiVersionLinodeInstancesLinodeIdVolumesResponseDataItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdVolumesResponseDataItemStatusEnum::Active => {
                "active"
            }
            GetApiVersionLinodeInstancesLinodeIdVolumesResponseDataItemStatusEnum::Creating => {
                "creating"
            }
            GetApiVersionLinodeInstancesLinodeIdVolumesResponseDataItemStatusEnum::KeyRotating => {
                "key_rotating"
            }
            GetApiVersionLinodeInstancesLinodeIdVolumesResponseDataItemStatusEnum::Resizing => {
                "resizing"
            }
        };
        write!(f, "{}", str_val)
    }
}
