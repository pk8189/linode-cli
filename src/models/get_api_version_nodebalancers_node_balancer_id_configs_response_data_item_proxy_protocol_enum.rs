#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionNodebalancersNodeBalancerIdConfigsResponseDataItemProxyProtocolEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "none"))]
    #[serde(rename = "none")]
    None,
    #[cfg_attr(feature = "cli", value(name = "v1"))]
    #[serde(rename = "v1")]
    V1,
    #[cfg_attr(feature = "cli", value(name = "v2"))]
    #[serde(rename = "v2")]
    V2,
}
impl std::fmt::Display
for GetApiVersionNodebalancersNodeBalancerIdConfigsResponseDataItemProxyProtocolEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionNodebalancersNodeBalancerIdConfigsResponseDataItemProxyProtocolEnum::None => {
                "none"
            }
            GetApiVersionNodebalancersNodeBalancerIdConfigsResponseDataItemProxyProtocolEnum::V1 => {
                "v1"
            }
            GetApiVersionNodebalancersNodeBalancerIdConfigsResponseDataItemProxyProtocolEnum::V2 => {
                "v2"
            }
        };
        write!(f, "{}", str_val)
    }
}
