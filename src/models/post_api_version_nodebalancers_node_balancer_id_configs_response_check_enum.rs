#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionNodebalancersNodeBalancerIdConfigsResponseCheckEnum {
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
for PostApiVersionNodebalancersNodeBalancerIdConfigsResponseCheckEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionNodebalancersNodeBalancerIdConfigsResponseCheckEnum::Connection => {
                "connection"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsResponseCheckEnum::Http => {
                "http"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsResponseCheckEnum::HttpBody => {
                "http_body"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsResponseCheckEnum::None => {
                "none"
            }
        };
        write!(f, "{}", str_val)
    }
}
