#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevices {
    #[cfg_attr(feature = "cli", arg(id = "sda", long = "sda"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSda>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sda: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSda,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdb", long = "sdb"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdb>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdb: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdb,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdc", long = "sdc"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdc>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdc: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdc,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdd", long = "sdd"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdd>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdd: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdd,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sde", long = "sde"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSde>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sde: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSde,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdf", long = "sdf"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdf>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdf: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdf,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdg", long = "sdg"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdg>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdg: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdg,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdh", long = "sdh"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdh>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdh: Option<
        crate::models::PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdBodyDevicesSdh,
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
