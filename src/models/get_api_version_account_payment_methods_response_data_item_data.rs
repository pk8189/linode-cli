#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GetApiVersionAccountPaymentMethodsResponseDataItemData {
    GetApiVersionAccountPaymentMethodsResponseDataItemDataObj0(
        crate::models::GetApiVersionAccountPaymentMethodsResponseDataItemDataObj0,
    ),
    GetApiVersionAccountPaymentMethodsResponseDataItemDataObj1(
        crate::models::GetApiVersionAccountPaymentMethodsResponseDataItemDataObj1,
    ),
    GetApiVersionAccountPaymentMethodsResponseDataItemDataObj2(
        crate::models::GetApiVersionAccountPaymentMethodsResponseDataItemDataObj2,
    ),
}
impl Default for GetApiVersionAccountPaymentMethodsResponseDataItemData {
    fn default() -> Self {
        GetApiVersionAccountPaymentMethodsResponseDataItemData::GetApiVersionAccountPaymentMethodsResponseDataItemDataObj1(
            Default::default(),
        )
    }
}
