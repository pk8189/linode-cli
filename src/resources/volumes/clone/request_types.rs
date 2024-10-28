#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionVolumesVolumeIdCloneApiVersionEnum,
    #[cfg_attr(feature = "cli", arg(id = "volume-id", long = "volume-id"))]
    pub volume_id: i64,
    #[cfg_attr(feature = "cli", command(flatten))]
    pub data: crate::models::PostApiVersionVolumesVolumeIdCloneBody,
}
