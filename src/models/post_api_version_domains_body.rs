#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionDomainsBody {
    #[cfg_attr(feature = "cli", arg(id = "axfr-ips", long = "axfr-ips"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axfr_ips: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "description", long = "description"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "domain", long = "domain"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "expire-sec", long = "expire-sec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_sec: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "group", long = "group"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "master-ips", long = "master-ips"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_ips: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "refresh-sec", long = "refresh-sec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_sec: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "retry-sec", long = "retry-sec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_sec: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "soa-email", long = "soa-email"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soa_email: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "status", long = "status"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::PostApiVersionDomainsBodyStatusEnum>,
    #[cfg_attr(feature = "cli", arg(id = "tags", long = "tags"))]
    #[cfg_attr(feature = "cli", arg(num_args = 0.., value_delimiter = ' '))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[cfg_attr(feature = "cli", arg(id = "ttl-sec", long = "ttl-sec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_sec: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "type-field", long = "type-field"))]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_field: Option<crate::models::PostApiVersionDomainsBodyTypeEnum>,
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
