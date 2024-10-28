#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdVolumesResponseDataItemHardwareTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "hdd"))]
    #[serde(rename = "hdd")]
    Hdd,
    #[cfg_attr(feature = "cli", value(name = "nvme"))]
    #[serde(rename = "nvme")]
    Nvme,
}
impl std::fmt::Display
for GetApiVersionLinodeInstancesLinodeIdVolumesResponseDataItemHardwareTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdVolumesResponseDataItemHardwareTypeEnum::Hdd => {
                "hdd"
            }
            GetApiVersionLinodeInstancesLinodeIdVolumesResponseDataItemHardwareTypeEnum::Nvme => {
                "nvme"
            }
        };
        write!(f, "{}", str_val)
    }
}
