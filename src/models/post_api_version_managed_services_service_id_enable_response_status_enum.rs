#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionManagedServicesServiceIdEnableResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "disabled"))]
    #[serde(rename = "disabled")]
    Disabled,
    #[cfg_attr(feature = "cli", value(name = "ok"))]
    #[serde(rename = "ok")]
    Ok,
    #[cfg_attr(feature = "cli", value(name = "pending"))]
    #[serde(rename = "pending")]
    Pending,
    #[cfg_attr(feature = "cli", value(name = "problem"))]
    #[serde(rename = "problem")]
    Problem,
}
impl std::fmt::Display
for PostApiVersionManagedServicesServiceIdEnableResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionManagedServicesServiceIdEnableResponseStatusEnum::Disabled => {
                "disabled"
            }
            PostApiVersionManagedServicesServiceIdEnableResponseStatusEnum::Ok => "ok",
            PostApiVersionManagedServicesServiceIdEnableResponseStatusEnum::Pending => {
                "pending"
            }
            PostApiVersionManagedServicesServiceIdEnableResponseStatusEnum::Problem => {
                "problem"
            }
        };
        write!(f, "{}", str_val)
    }
}
