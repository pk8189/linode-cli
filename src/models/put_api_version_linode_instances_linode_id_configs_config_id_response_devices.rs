#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevices {
    #[cfg_attr(feature = "cli", arg(id = "sda", long = "sda"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSda>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sda: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSda,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdb", long = "sdb"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdb>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdb: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdb,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdc", long = "sdc"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdc>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdc: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdc,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdd", long = "sdd"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdd>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdd: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdd,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sde", long = "sde"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSde>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sde: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSde,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdf", long = "sdf"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdf>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdf: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdf,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdg", long = "sdg"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdg>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdg: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdg,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdh", long = "sdh"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdh>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdh: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdResponseDevicesSdh,
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
