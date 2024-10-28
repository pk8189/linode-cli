#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "billing_suspension"))]
    #[serde(rename = "billing_suspension")]
    BillingSuspension,
    #[cfg_attr(feature = "cli", value(name = "booting"))]
    #[serde(rename = "booting")]
    Booting,
    #[cfg_attr(feature = "cli", value(name = "busy"))]
    #[serde(rename = "busy")]
    Busy,
    #[cfg_attr(feature = "cli", value(name = "cloning"))]
    #[serde(rename = "cloning")]
    Cloning,
    #[cfg_attr(feature = "cli", value(name = "deleting"))]
    #[serde(rename = "deleting")]
    Deleting,
    #[cfg_attr(feature = "cli", value(name = "migrating"))]
    #[serde(rename = "migrating")]
    Migrating,
    #[cfg_attr(feature = "cli", value(name = "offline"))]
    #[serde(rename = "offline")]
    Offline,
    #[cfg_attr(feature = "cli", value(name = "provisioning"))]
    #[serde(rename = "provisioning")]
    Provisioning,
    #[cfg_attr(feature = "cli", value(name = "rebooting"))]
    #[serde(rename = "rebooting")]
    Rebooting,
    #[cfg_attr(feature = "cli", value(name = "rebuilding"))]
    #[serde(rename = "rebuilding")]
    Rebuilding,
    #[cfg_attr(feature = "cli", value(name = "restoring"))]
    #[serde(rename = "restoring")]
    Restoring,
    #[cfg_attr(feature = "cli", value(name = "running"))]
    #[serde(rename = "running")]
    Running,
    #[cfg_attr(feature = "cli", value(name = "shutting_down"))]
    #[serde(rename = "shutting_down")]
    ShuttingDown,
    #[cfg_attr(feature = "cli", value(name = "stopped"))]
    #[serde(rename = "stopped")]
    Stopped,
}
impl std::fmt::Display for GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::BillingSuspension => {
                "billing_suspension"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Booting => {
                "booting"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Busy => "busy",
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Cloning => {
                "cloning"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Deleting => {
                "deleting"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Migrating => {
                "migrating"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Offline => {
                "offline"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Provisioning => {
                "provisioning"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Rebooting => {
                "rebooting"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Rebuilding => {
                "rebuilding"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Restoring => {
                "restoring"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Running => {
                "running"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::ShuttingDown => {
                "shutting_down"
            }
            GetApiVersionTagsTagLabelResponseDataItemDataObj0StatusEnum::Stopped => {
                "stopped"
            }
        };
        write!(f, "{}", str_val)
    }
}
