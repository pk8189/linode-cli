#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionNodebalancersNodeBalancerIdConfigsResponseProtocolEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "http"))]
    #[serde(rename = "http")]
    Http,
    #[cfg_attr(feature = "cli", value(name = "https"))]
    #[serde(rename = "https")]
    Https,
    #[cfg_attr(feature = "cli", value(name = "tcp"))]
    #[serde(rename = "tcp")]
    Tcp,
}
impl std::fmt::Display
for PostApiVersionNodebalancersNodeBalancerIdConfigsResponseProtocolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionNodebalancersNodeBalancerIdConfigsResponseProtocolEnum::Http => {
                "http"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsResponseProtocolEnum::Https => {
                "https"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsResponseProtocolEnum::Tcp => {
                "tcp"
            }
        };
        write!(f, "{}", str_val)
    }
}
