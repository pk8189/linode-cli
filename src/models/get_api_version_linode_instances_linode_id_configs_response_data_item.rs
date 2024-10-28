#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "comments", long = "comments"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub comments: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "devices", long = "devices"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevices>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemDevices,
    >,
    #[cfg_attr(feature = "cli", arg(id = "helpers", long = "helpers"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemHelpers>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helpers: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemHelpers,
    >,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "interfaces", long = "interfaces"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemInterfacesItem>
        )
    )]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interfaces: Option<
        Vec<
            crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemInterfacesItem,
        >,
    >,
    #[cfg_attr(feature = "cli", arg(id = "kernel", long = "kernel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "memory-limit", long = "memory-limit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "root-device", long = "root-device"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_device: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "run-level", long = "run-level"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_level: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemRunLevelEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "virt-mode", long = "virt-mode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virt_mode: Option<
        crate::models::GetApiVersionLinodeInstancesLinodeIdConfigsResponseDataItemVirtModeEnum,
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
