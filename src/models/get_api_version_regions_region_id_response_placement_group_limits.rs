#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionRegionsRegionIdResponsePlacementGroupLimits {
    #[cfg_attr(
        feature = "cli",
        arg(id = "maximum-linodes-per-pg", long = "maximum-linodes-per-pg")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_linodes_per_pg: Option<i64>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "maximum-pgs-per-customer", long = "maximum-pgs-per-customer")
    )]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<i64>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub maximum_pgs_per_customer: crate::core::patch::Patch<i64>,
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
