#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeInstancesLinodeIdRescueBodyDevices {
    #[cfg_attr(feature = "cli", arg(id = "sda", long = "sda"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSda>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sda: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSda,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdb", long = "sdb"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdb>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdb: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdb,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdc", long = "sdc"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdc>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdc: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdc,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdd", long = "sdd"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdd>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdd: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdd,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sde", long = "sde"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSde>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sde: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSde,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdf", long = "sdf"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdf>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdf: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdf,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdg", long = "sdg"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdg>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdg: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdRescueBodyDevicesSdg,
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
