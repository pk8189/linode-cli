#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionTagsTagLabelResponseDataItemDataObj2HardwareTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "hdd"))]
    #[serde(rename = "hdd")]
    Hdd,
    #[cfg_attr(feature = "cli", value(name = "nvme"))]
    #[serde(rename = "nvme")]
    Nvme,
}
impl std::fmt::Display
for GetApiVersionTagsTagLabelResponseDataItemDataObj2HardwareTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionTagsTagLabelResponseDataItemDataObj2HardwareTypeEnum::Hdd => {
                "hdd"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj2HardwareTypeEnum::Nvme => {
                "nvme"
            }
        };
        write!(f, "{}", str_val)
    }
}
