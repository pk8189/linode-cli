#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseModeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "accept"))]
    #[serde(rename = "accept")]
    Accept,
    #[cfg_attr(feature = "cli", value(name = "backup"))]
    #[serde(rename = "backup")]
    Backup,
    #[cfg_attr(feature = "cli", value(name = "drain"))]
    #[serde(rename = "drain")]
    Drain,
    #[cfg_attr(feature = "cli", value(name = "reject"))]
    #[serde(rename = "reject")]
    Reject,
}
impl std::fmt::Display
for PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseModeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseModeEnum::Accept => {
                "accept"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseModeEnum::Backup => {
                "backup"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseModeEnum::Drain => {
                "drain"
            }
            PostApiVersionNodebalancersNodeBalancerIdConfigsConfigIdNodesResponseModeEnum::Reject => {
                "reject"
            }
        };
        write!(f, "{}", str_val)
    }
}
