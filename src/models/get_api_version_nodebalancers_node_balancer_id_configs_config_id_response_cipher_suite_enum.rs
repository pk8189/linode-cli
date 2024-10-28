#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseCipherSuiteEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "legacy"))]
    #[serde(rename = "legacy")]
    Legacy,
    #[cfg_attr(feature = "cli", value(name = "recommended"))]
    #[serde(rename = "recommended")]
    Recommended,
}
impl std::fmt::Display
for GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseCipherSuiteEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseCipherSuiteEnum::Legacy => {
                "legacy"
            }
            GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseCipherSuiteEnum::Recommended => {
                "recommended"
            }
        };
        write!(f, "{}", str_val)
    }
}
