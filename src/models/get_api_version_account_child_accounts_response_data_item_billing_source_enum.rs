#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountChildAccountsResponseDataItemBillingSourceEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "external"))]
    #[serde(rename = "external")]
    External,
}
impl std::fmt::Display
for GetApiVersionAccountChildAccountsResponseDataItemBillingSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountChildAccountsResponseDataItemBillingSourceEnum::External => {
                "external"
            }
        };
        write!(f, "{}", str_val)
    }
}
