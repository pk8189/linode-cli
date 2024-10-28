#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountLoginsLoginIdResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "failed"))]
    #[serde(rename = "failed")]
    Failed,
    #[cfg_attr(feature = "cli", value(name = "successful"))]
    #[serde(rename = "successful")]
    Successful,
}
impl std::fmt::Display for GetApiVersionAccountLoginsLoginIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountLoginsLoginIdResponseStatusEnum::Failed => "failed",
            GetApiVersionAccountLoginsLoginIdResponseStatusEnum::Successful => {
                "successful"
            }
        };
        write!(f, "{}", str_val)
    }
}
