#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionAccountPaymentMethodsBodyTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "credit_card"))]
    #[serde(rename = "credit_card")]
    CreditCard,
}
impl std::fmt::Display for PostApiVersionAccountPaymentMethodsBodyTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionAccountPaymentMethodsBodyTypeEnum::CreditCard => "credit_card",
        };
        write!(f, "{}", str_val)
    }
}
