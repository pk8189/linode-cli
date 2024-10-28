#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct CreateRequest {
    #[cfg_attr(feature = "cli", arg(id = "api-version", long = "api-version"))]
    pub api_version: crate::models::PostApiVersionAccountPaymentMethodsPaymentMethodIdMakeDefaultApiVersionEnum,
    #[cfg_attr(
        feature = "cli",
        arg(id = "payment-method-id", long = "payment-method-id")
    )]
    pub payment_method_id: i64,
}
