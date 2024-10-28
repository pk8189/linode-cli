#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "all"))]
    #[serde(rename = "all")]
    All,
    #[cfg_attr(feature = "cli", value(name = "backup"))]
    #[serde(rename = "backup")]
    Backup,
    #[cfg_attr(feature = "cli", value(name = "blockstorage"))]
    #[serde(rename = "blockstorage")]
    Blockstorage,
    #[cfg_attr(feature = "cli", value(name = "db_mysql"))]
    #[serde(rename = "db_mysql")]
    DbMysql,
    #[cfg_attr(feature = "cli", value(name = "ip_v4"))]
    #[serde(rename = "ip_v4")]
    IpV4,
    #[cfg_attr(feature = "cli", value(name = "linode"))]
    #[serde(rename = "linode")]
    Linode,
    #[cfg_attr(feature = "cli", value(name = "linode_disk"))]
    #[serde(rename = "linode_disk")]
    LinodeDisk,
    #[cfg_attr(feature = "cli", value(name = "linode_memory"))]
    #[serde(rename = "linode_memory")]
    LinodeMemory,
    #[cfg_attr(feature = "cli", value(name = "longview"))]
    #[serde(rename = "longview")]
    Longview,
    #[cfg_attr(feature = "cli", value(name = "managed"))]
    #[serde(rename = "managed")]
    Managed,
    #[cfg_attr(feature = "cli", value(name = "nodebalancer"))]
    #[serde(rename = "nodebalancer")]
    Nodebalancer,
    #[cfg_attr(feature = "cli", value(name = "objectstorage"))]
    #[serde(rename = "objectstorage")]
    Objectstorage,
    #[cfg_attr(feature = "cli", value(name = "placement_group"))]
    #[serde(rename = "placement_group")]
    PlacementGroup,
    #[cfg_attr(feature = "cli", value(name = "transfer_tx"))]
    #[serde(rename = "transfer_tx")]
    TransferTx,
}
impl std::fmt::Display
for GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::All => "all",
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::Backup => {
                "backup"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::Blockstorage => {
                "blockstorage"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::DbMysql => {
                "db_mysql"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::IpV4 => {
                "ip_v4"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::Linode => {
                "linode"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::LinodeDisk => {
                "linode_disk"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::LinodeMemory => {
                "linode_memory"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::Longview => {
                "longview"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::Managed => {
                "managed"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::Nodebalancer => {
                "nodebalancer"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::Objectstorage => {
                "objectstorage"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::PlacementGroup => {
                "placement_group"
            }
            GetApiVersionAccountResponseActivePromotionsItemServiceTypeEnum::TransferTx => {
                "transfer_tx"
            }
        };
        write!(f, "{}", str_val)
    }
}
