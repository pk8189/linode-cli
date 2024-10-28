#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponse {
    #[cfg_attr(feature = "cli", arg(id = "algorithm", long = "algorithm"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<
        crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponseAlgorithmEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "check", long = "check"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check: Option<
        crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponseCheckEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "check-attempts", long = "check-attempts"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_attempts: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "check-body", long = "check-body"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_body: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "check-interval", long = "check-interval"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_interval: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "check-passive", long = "check-passive"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_passive: Option<bool>,
    #[cfg_attr(feature = "cli", arg(id = "check-path", long = "check-path"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_path: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "check-timeout", long = "check-timeout"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_timeout: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "cipher-suite", long = "cipher-suite"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cipher_suite: Option<
        crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponseCipherSuiteEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "id", long = "id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "nodebalancer-id", long = "nodebalancer-id"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodebalancer_id: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "nodes-status", long = "nodes-status"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_json::<crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponseNodesStatus>
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes_status: Option<
        crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponseNodesStatus,
    >,
    #[cfg_attr(feature = "cli", arg(id = "port", long = "port"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[cfg_attr(feature = "cli", arg(id = "protocol", long = "protocol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<
        crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponseProtocolEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "proxy-protocol", long = "proxy-protocol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_protocol: Option<
        crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponseProxyProtocolEnum,
    >,
    #[cfg_attr(feature = "cli", arg(id = "ssl-cert", long = "ssl-cert"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub ssl_cert: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "ssl-commonname", long = "ssl-commonname"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_commonname: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "ssl-fingerprint", long = "ssl-fingerprint"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_fingerprint: Option<String>,
    #[cfg_attr(feature = "cli", arg(id = "ssl-key", long = "ssl-key"))]
    #[cfg_attr(
        feature = "cli",
        arg(
            value_parser = crate::core::clap::parse_patch::<String>,
            default_value = "___undefined___"
        )
    )]
    #[serde(default)]
    #[serde(skip_serializing_if = "crate::core::patch::Patch::is_undefined")]
    pub ssl_key: crate::core::patch::Patch<String>,
    #[cfg_attr(feature = "cli", arg(id = "stickiness", long = "stickiness"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickiness: Option<
        crate::models::PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdRebuildResponseStickinessEnum,
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
