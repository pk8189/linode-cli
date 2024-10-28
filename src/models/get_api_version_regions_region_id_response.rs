#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionRegionsRegionIdResponse {
    #[cfg_attr(feature = "cli", arg(id = "capabilities", long = "capabilities"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "country", long = "country"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(
        feature = "cli",
        arg(id = "placement-group-limits", long = "placement-group-limits")
    )]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionRegionsRegionIdResponsePlacementGroupLimits>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group_limits: Option<
        crate::models::GetApiVersionRegionsRegionIdResponsePlacementGroupLimits,
    >,
    #[cfg_attr(feature = "cli", arg(id = "resolvers", long = "resolvers"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionRegionsRegionIdResponseResolvers>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolvers: Option<crate::models::GetApiVersionRegionsRegionIdResponseResolvers>,
    #[cfg_attr(feature = "cli", arg(id = "site-type", long = "site-type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_type: Option<
        crate::models::GetApiVersionRegionsRegionIdResponseSiteTypeEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::GetApiVersionRegionsRegionIdResponseStatusEnum>,
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
