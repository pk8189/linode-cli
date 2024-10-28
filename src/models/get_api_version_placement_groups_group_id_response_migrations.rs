#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionPlacementGroupsGroupIdResponseMigrations {
    #[cfg_attr(feature = "cli", arg(id = "inbound", long = "inbound"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionPlacementGroupsGroupIdResponseMigrationsInboundItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound: Option<
        Vec<
            crate::models::GetApiVersionPlacementGroupsGroupIdResponseMigrationsInboundItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "outbound", long = "outbound"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionPlacementGroupsGroupIdResponseMigrationsOutboundItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound: Option<
        Vec<
            crate::models::GetApiVersionPlacementGroupsGroupIdResponseMigrationsOutboundItem,
        >,
    >,
}
