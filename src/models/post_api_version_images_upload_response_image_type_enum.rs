#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionImagesUploadResponseImageTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "automatic"))]
    #[serde(rename = "automatic")]
    Automatic,
    #[cfg_attr(feature = "cli", value(name = "manual"))]
    #[serde(rename = "manual")]
    Manual,
}
impl std::fmt::Display for PostApiVersionImagesUploadResponseImageTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionImagesUploadResponseImageTypeEnum::Automatic => "automatic",
            PostApiVersionImagesUploadResponseImageTypeEnum::Manual => "manual",
        };
        write!(f, "{}", str_val)
    }
}
