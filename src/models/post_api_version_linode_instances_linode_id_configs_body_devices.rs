#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevices {
    #[cfg_attr(feature = "cli", arg(id = "sda", long = "sda"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSda>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sda: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSda,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdb", long = "sdb"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdb>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdb: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdb,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdc", long = "sdc"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdc>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdc: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdc,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdd", long = "sdd"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdd>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdd: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdd,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sde", long = "sde"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSde>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sde: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSde,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdf", long = "sdf"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdf>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdf: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdf,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdg", long = "sdg"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdg>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdg: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdg,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdh", long = "sdh"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdh>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdh: Option<
        crate::models::PostApiVersionLinodeInstancesLinodeIdConfigsBodyDevicesSdh,
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
