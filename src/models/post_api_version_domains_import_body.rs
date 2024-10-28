#[cfg_attr(feature = "cli", derive(clap::Args))]
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct PostApiVersionDomainsImportBody {
    #[cfg_attr(feature = "cli", arg(id = "domain", long = "domain"))]
    pub domain: String,
    #[cfg_attr(
        feature = "cli",
        arg(id = "remote-nameserver", long = "remote-nameserver")
    )]
    pub remote_nameserver: String,
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
