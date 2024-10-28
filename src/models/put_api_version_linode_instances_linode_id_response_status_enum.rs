#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum {
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
impl std::fmt::Display for PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::BillingSuspension => {
                "billing_suspension"
            }
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Booting => "booting",
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Busy => "busy",
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Cloning => "cloning",
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Deleting => {
                "deleting"
            }
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Migrating => {
                "migrating"
            }
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Offline => "offline",
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Provisioning => {
                "provisioning"
            }
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Rebooting => {
                "rebooting"
            }
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Rebuilding => {
                "rebuilding"
            }
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Restoring => {
                "restoring"
            }
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Running => "running",
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::ShuttingDown => {
                "shutting_down"
            }
            PutApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Stopped => "stopped",
        };
        write!(f, "{}", str_val)
    }
}
