#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountInvoicesInvoiceIdResponseBillingSourceEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "akamai"))]
    #[serde(rename = "akamai")]
    Akamai,
    #[cfg_attr(feature = "cli", value(name = "linode"))]
    #[serde(rename = "linode")]
    Linode,
}
impl std::fmt::Display
for GetApiVersionAccountInvoicesInvoiceIdResponseBillingSourceEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountInvoicesInvoiceIdResponseBillingSourceEnum::Akamai => {
                "akamai"
            }
            GetApiVersionAccountInvoicesInvoiceIdResponseBillingSourceEnum::Linode => {
                "linode"
            }
        };
        write!(f, "{}", str_val)
    }
}
