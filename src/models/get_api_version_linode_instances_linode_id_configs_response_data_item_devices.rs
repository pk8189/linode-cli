#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevices {
    #[cfg_attr(feature = "cli", arg(id = "sda", long = "sda"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSda>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sda: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSda,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdb", long = "sdb"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdb>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdb: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdb,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdc", long = "sdc"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdc>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdc: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdc,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdd", long = "sdd"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdd>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdd: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdd,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sde", long = "sde"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSde>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sde: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSde,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdf", long = "sdf"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdf>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdf: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdf,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdg", long = "sdg"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdg>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdg: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdg,
    >,
    #[cfg_attr(feature = "cli", arg(id = "sdh", long = "sdh"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdh>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdh: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevicesSdh,
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
