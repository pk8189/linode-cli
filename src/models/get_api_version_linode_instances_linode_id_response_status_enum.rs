#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum {
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
impl std::fmt::Display for GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::BillingSuspension => {
                "billing_suspension"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Booting => "booting",
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Busy => "busy",
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Cloning => "cloning",
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Deleting => {
                "deleting"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Migrating => {
                "migrating"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Offline => "offline",
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Provisioning => {
                "provisioning"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Rebooting => {
                "rebooting"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Rebuilding => {
                "rebuilding"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Restoring => {
                "restoring"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Running => "running",
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::ShuttingDown => {
                "shutting_down"
            }
            GetApiVersionLinodeInstancesLinodeIdResponseStatusEnum::Stopped => "stopped",
        };
        write!(f, "{}", str_val)
    }
}
