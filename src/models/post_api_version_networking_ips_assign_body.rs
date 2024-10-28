#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionNetworkingIpsAssignBody {
    #[cfg_attr(feature = "cli", arg(id = "assignments", long = "assignments"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionNetworkingIpsAssignBodyAssignmentsItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    pub assignments: Vec<
        crate::models::PostApiVersionNetworkingIpsAssignBodyAssignmentsItem,
    >,
    #[cfg_attr(feature = "cli", arg(id = "region", long = "region"))]
    pub region: String,
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
