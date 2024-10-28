#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdStatsResponse {
    #[cfg_attr(feature = "cli", arg(id = "cpu", long = "cpu"))]
    #[cfg_attr(
        feature = "cli",
        arg(value_parser = crate::core::clap::parse_json::<Vec<f64>>)
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<Vec<Vec<f64>>>,
    #[cfg_attr(feature = "cli", arg(id = "io", long = "io"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdStatsResponseIo>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub io: Option<crate::models::GetApiVersionLinodeInstancesLinodeIdStatsResponseIo>,
    #[cfg_attr(feature = "cli", arg(id = "netv4", long = "netv4"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdStatsResponseNetv4>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netv4: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdStatsResponseNetv4,
    >,
    #[cfg_attr(feature = "cli", arg(id = "netv6", long = "netv6"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdStatsResponseNetv6>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netv6: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdStatsResponseNetv6,
    >,
    #[cfg_attr(feature = "cli", arg(id = "title", long = "title"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
