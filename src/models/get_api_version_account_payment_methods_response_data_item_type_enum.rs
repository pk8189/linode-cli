#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountPaymentMethodsResponseDataItemTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "credit_card"))]
    #[serde(rename = "credit_card")]
    CreditCard,
    #[cfg_attr(feature = "cli", value(name = "google_pay"))]
    #[serde(rename = "google_pay")]
    GooglePay,
    #[cfg_attr(feature = "cli", value(name = "paypal"))]
    #[serde(rename = "paypal")]
    Paypal,
}
impl std::fmt::Display for GetApiVersionAccountPaymentMethodsResponseDataItemTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountPaymentMethodsResponseDataItemTypeEnum::CreditCard => {
                "credit_card"
            }
            GetApiVersionAccountPaymentMethodsResponseDataItemTypeEnum::GooglePay => {
                "google_pay"
            }
            GetApiVersionAccountPaymentMethodsResponseDataItemTypeEnum::Paypal => {
                "paypal"
            }
        };
        write!(f, "{}", str_val)
    }
}
