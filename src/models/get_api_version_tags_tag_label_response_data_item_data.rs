#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GetApiVersionTagsTagLabelResponseDataItemData {
    GetApiVersionTagsTagLabelResponseDataItemDataObj0(
        crate::models::GetApiVersionTagsTagLabelResponseDataItemDataObj0,
    ),
    GetApiVersionTagsTagLabelResponseDataItemDataObj1(
        crate::models::GetApiVersionTagsTagLabelResponseDataItemDataObj1,
    ),
    GetApiVersionTagsTagLabelResponseDataItemDataObj2(
        crate::models::GetApiVersionTagsTagLabelResponseDataItemDataObj2,
    ),
    GetApiVersionTagsTagLabelResponseDataItemDataObj3(
        crate::models::GetApiVersionTagsTagLabelResponseDataItemDataObj3,
    ),
}
impl Default for GetApiVersionTagsTagLabelResponseDataItemData {
    fn default() -> Self {
        GetApiVersionTagsTagLabelResponseDataItemData::GetApiVersionTagsTagLabelResponseDataItemDataObj1(
            Default::default(),
        )
    }
}
