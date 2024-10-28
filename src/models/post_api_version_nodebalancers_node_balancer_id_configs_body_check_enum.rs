#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionNodebalancersNodeBalancerIdConfigsBodyCheckEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "connection"))]
    #[serde(rename = "connection")]
    Connection,
    #[cfg_attr(feature = "cli", value(name = "http"))]
    #[serde(rename = "http")]
    Http,
    #[cfg_attr(feature = "cli", value(name = "http_body"))]
    #[serde(rename = "http_body")]
    HttpBody,
    #[cfg_attr(feature = "cli", value(name = "none"))]
    #[serde(rename = "none")]
    None,
}
impl std::fmt::Display
for PostApiVersionNodebalancersNodeBalancerIdConfigsBodyCheckEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionNodebalancersNodeBalancerIdConfigsBodyCheckEnum::Connection => {
                "connection"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsBodyCheckEnum::Http => "http",
            PostApiVersionNodebalancersNodeBalancerIdConfigsBodyCheckEnum::HttpBody => {
                "http_body"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsBodyCheckEnum::None => "none",
        };
        write!(f, "{}", str_val)
    }
}
