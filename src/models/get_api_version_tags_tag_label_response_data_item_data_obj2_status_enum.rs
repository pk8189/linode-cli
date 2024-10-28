#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionTagsTagLabelResponseDataItemDataObj2StatusEnum {
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
impl std::fmt::Display for GetApiVersionTagsTagLabelResponseDataItemDataObj2StatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionTagsTagLabelResponseDataItemDataObj2StatusEnum::Active => {
                "active"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj2StatusEnum::Creating => {
                "creating"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj2StatusEnum::KeyRotating => {
                "key_rotating"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj2StatusEnum::Resizing => {
                "resizing"
            }
        };
        write!(f, "{}", str_val)
    }
}
