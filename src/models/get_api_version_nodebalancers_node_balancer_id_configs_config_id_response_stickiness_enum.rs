#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseStickinessEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "http_cookie"))]
    #[serde(rename = "http_cookie")]
    HttpCookie,
    #[cfg_attr(feature = "cli", value(name = "none"))]
    #[serde(rename = "none")]
    None,
    #[cfg_attr(feature = "cli", value(name = "table"))]
    #[serde(rename = "table")]
    Table,
}
impl std::fmt::Display
for GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseStickinessEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseStickinessEnum::HttpCookie => {
                "http_cookie"
            }
            GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseStickinessEnum::None => {
                "none"
            }
            GetApiVersionNodebalancersNodeBalancerIdConfigsConfigIdResponseStickinessEnum::Table => {
                "table"
            }
        };
        write!(f, "{}", str_val)
    }
}
