#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum GetApiVersionManagedStatsResponseData {
    GetApiVersionManagedStatsResponseDataObj0(
        crate::models::GetApiVersionManagedStatsResponseDataObj0,
    ),
    ArrString(Vec<String>),
}
impl Default for GetApiVersionManagedStatsResponseData {
    fn default() -> Self {
        GetApiVersionManagedStatsResponseData::ArrString(Default::default())
    }
}
