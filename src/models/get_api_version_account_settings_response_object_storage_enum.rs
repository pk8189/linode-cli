#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountSettingsResponseObjectStorageEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "active"))]
    #[serde(rename = "active")]
    Active,
    #[cfg_attr(feature = "cli", value(name = "disabled"))]
    #[serde(rename = "disabled")]
    Disabled,
    #[cfg_attr(feature = "cli", value(name = "suspended"))]
    #[serde(rename = "suspended")]
    Suspended,
}
impl std::fmt::Display for GetApiVersionAccountSettingsResponseObjectStorageEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountSettingsResponseObjectStorageEnum::Active => "active",
            GetApiVersionAccountSettingsResponseObjectStorageEnum::Disabled => "disabled",
            GetApiVersionAccountSettingsResponseObjectStorageEnum::Suspended => {
                "suspended"
            }
        };
        write!(f, "{}", str_val)
    }
}
