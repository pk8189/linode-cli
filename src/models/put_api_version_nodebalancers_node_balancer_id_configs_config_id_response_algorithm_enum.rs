#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseAlgorithmEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "leastconn"))]
    #[serde(rename = "leastconn")]
    Leastconn,
    #[cfg_attr(feature = "cli", value(name = "roundrobin"))]
    #[serde(rename = "roundrobin")]
    Roundrobin,
    #[cfg_attr(feature = "cli", value(name = "source"))]
    #[serde(rename = "source")]
    Source,
}
impl std::fmt::Display
for PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseAlgorithmEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseAlgorithmEnum::Leastconn => {
                "leastconn"
            }
            PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseAlgorithmEnum::Roundrobin => {
                "roundrobin"
            }
            PutApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseAlgorithmEnum::Source => {
                "source"
            }
        };
        write!(f, "{}", str_val)
    }
}
