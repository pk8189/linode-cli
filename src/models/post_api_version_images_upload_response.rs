#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionImagesUploadResponse {
    #[cfg_attr(feature = "cli", arg(id = "image", long = "image"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionImagesUploadResponseImage>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<crate::models::PostApiVersionImagesUploadResponseImage>,
    #[cfg_attr(feature = "cli", arg(id = "upload-to", long = "upload-to"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_to: Option<String>,
    #[serde(flatten)]
    #[cfg_attr(
        feature = "cli",
        arg(
            id = "additional-props",
            long = "additional-props",
            value_parser = crate::core::clap::parse_json::<std::collections::HashMap<String,
            serde_json::Value>>,
            default_value = "{}",
        )
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
