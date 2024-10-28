#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseStatusEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "DOWN"))]
    #[serde(rename = "DOWN")]
    Down,
    #[cfg_attr(feature = "cli", value(name = "UP"))]
    #[serde(rename = "UP")]
    Up,
    #[cfg_attr(feature = "cli", value(name = "unknown"))]
    #[serde(rename = "unknown")]
    Unknown,
}
impl std::fmt::Display
for PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseStatusEnum::Down => {
                "DOWN"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseStatusEnum::Up => {
                "UP"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseStatusEnum::Unknown => {
                "unknown"
            }
        };
        write!(f, "{}", str_val)
    }
}
