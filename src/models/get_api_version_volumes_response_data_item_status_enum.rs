#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionVolumesResponseDataItemStatusEnum {
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
impl std::fmt::Display for GetApiVersionVolumesResponseDataItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionVolumesResponseDataItemStatusEnum::Active => "active",
            GetApiVersionVolumesResponseDataItemStatusEnum::Creating => "creating",
            GetApiVersionVolumesResponseDataItemStatusEnum::KeyRotating => "key_rotating",
            GetApiVersionVolumesResponseDataItemStatusEnum::Resizing => "resizing",
        };
        write!(f, "{}", str_val)
    }
}
