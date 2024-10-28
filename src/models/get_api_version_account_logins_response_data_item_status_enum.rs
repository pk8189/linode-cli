#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountLoginsResponseDataItemStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "failed"))]
    #[serde(rename = "failed")]
    Failed,
    #[cfg_attr(feature = "cli", value(name = "successful"))]
    #[serde(rename = "successful")]
    Successful,
}
impl std::fmt::Display for GetApiVersionAccountLoginsResponseDataItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountLoginsResponseDataItemStatusEnum::Failed => "failed",
            GetApiVersionAccountLoginsResponseDataItemStatusEnum::Successful => {
                "successful"
            }
        };
        write!(f, "{}", str_val)
    }
}
