#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressDisksItemFilesystemEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "ext3"))]
    #[serde(rename = "ext3")]
    Ext3,
    #[cfg_attr(feature = "cli", value(name = "ext4"))]
    #[serde(rename = "ext4")]
    Ext4,
    #[cfg_attr(feature = "cli", value(name = "initrd"))]
    #[serde(rename = "initrd")]
    Initrd,
    #[cfg_attr(feature = "cli", value(name = "raw"))]
    #[serde(rename = "raw")]
    Raw,
    #[cfg_attr(feature = "cli", value(name = "swap"))]
    #[serde(rename = "swap")]
    Swap,
}
impl std::fmt::Display
for GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressDisksItemFilesystemEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressDisksItemFilesystemEnum::Ext3 => {
                "ext3"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressDisksItemFilesystemEnum::Ext4 => {
                "ext4"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressDisksItemFilesystemEnum::Initrd => {
                "initrd"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressDisksItemFilesystemEnum::Raw => {
                "raw"
            }
            GetApiVersionLinodeInstancesLinodeIdBackupsResponseSnapshotInProgressDisksItemFilesystemEnum::Swap => {
                "swap"
            }
        };
        write!(f, "{}", str_val)
    }
}
