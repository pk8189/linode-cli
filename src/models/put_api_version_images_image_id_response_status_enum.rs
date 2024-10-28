#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionImagesImageIdResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "available"))]
    #[serde(rename = "available")]
    Available,
    #[cfg_attr(feature = "cli", value(name = "creating"))]
    #[serde(rename = "creating")]
    Creating,
    #[cfg_attr(feature = "cli", value(name = "pending_upload"))]
    #[serde(rename = "pending_upload")]
    PendingUpload,
}
impl std::fmt::Display for PutApiVersionImagesImageIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionImagesImageIdResponseStatusEnum::Available => "available",
            PutApiVersionImagesImageIdResponseStatusEnum::Creating => "creating",
            PutApiVersionImagesImageIdResponseStatusEnum::PendingUpload => {
                "pending_upload"
            }
        };
        write!(f, "{}", str_val)
    }
}
