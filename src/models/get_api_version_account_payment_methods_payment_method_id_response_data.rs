#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseData {
    GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseDataObj0(
        crate::models::GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseDataObj0,
    ),
    GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseDataObj1(
        crate::models::GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseDataObj1,
    ),
    GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseDataObj2(
        crate::models::GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseDataObj2,
    ),
}
impl Default for GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseData {
    fn default() -> Self {
        GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseData::GetApiVersionAccountPaymentMethodsPaymentMethodIdResponseDataObj1(
            Default::default(),
        )
    }
}
