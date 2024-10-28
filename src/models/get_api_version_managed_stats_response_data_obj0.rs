#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionManagedStatsResponseDataObj0 {
    #[cfg_attr(feature = "cli", arg(id = "cpu", long = "cpu"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionManagedStatsResponseDataObj0CpuItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<
        Vec<crate::models::GetApiVersionManagedStatsResponseDataObj0CpuItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "disk", long = "disk"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionManagedStatsResponseDataObj0DiskItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<
        Vec<crate::models::GetApiVersionManagedStatsResponseDataObj0DiskItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "net-in", long = "net-in"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionManagedStatsResponseDataObj0NetInItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_in: Option<
        Vec<crate::models::GetApiVersionManagedStatsResponseDataObj0NetInItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "net-out", long = "net-out"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionManagedStatsResponseDataObj0NetOutItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_out: Option<
        Vec<crate::models::GetApiVersionManagedStatsResponseDataObj0NetOutItem>,
    >,
    #[cfg_attr(feature = "cli", arg(id = "swap", long = "swap"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionManagedStatsResponseDataObj0SwapItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<
        Vec<crate::models::GetApiVersionManagedStatsResponseDataObj0SwapItem>,
    >,
    #[serde(flatten)]
    #[cfg_attr(
        feature = "cli",
        arg(
            id = "additional-props",
            long = "additional-props",
            value_parser = crate::core::clap::parse_json::<std::collections::HashMap<String,
            serde_json::Value>>,
            default_value = "{}",
        )
    )]
    pub additional_properties: std::collections::HashMap<String, serde_json::Value>,
}
