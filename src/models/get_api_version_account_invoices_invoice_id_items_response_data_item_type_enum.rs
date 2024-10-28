#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountInvoicesInvoiceIdItemsResponseDataItemTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "hourly"))]
    #[serde(rename = "hourly")]
    Hourly,
    #[cfg_attr(feature = "cli", value(name = "misc"))]
    #[serde(rename = "misc")]
    Misc,
}
impl std::fmt::Display
for GetApiVersionAccountInvoicesInvoiceIdItemsResponseDataItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountInvoicesInvoiceIdItemsResponseDataItemTypeEnum::Hourly => {
                "hourly"
            }
            GetApiVersionAccountInvoicesInvoiceIdItemsResponseDataItemTypeEnum::Misc => {
                "misc"
            }
        };
        write!(f, "{}", str_val)
    }
}
