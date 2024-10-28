#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct GetApiVersionLinodeKernelsResponseDataItem {
    #[cfg_attr(feature = "cli", arg(id = "architecture", long = "architecture"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<
        crate::models::GetApiVersionLinodeKernelsResponseDataItemArchitectureEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "built", long = "built"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "deprecated", long = "deprecated"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "kvm", long = "kvm"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kvm: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "label", long = "label"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "pvops", long = "pvops"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pvops: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "version", long = "version"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
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
