#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionNodebalancersBodyConfigsItemNodesItemStatusEnum {
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
for PostApiVersionNodebalancersBodyConfigsItemNodesItemStatusEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionNodebalancersBodyConfigsItemNodesItemStatusEnum::Down => "DOWN",
            PostApiVersionNodebalancersBodyConfigsItemNodesItemStatusEnum::Up => "UP",
            PostApiVersionNodebalancersBodyConfigsItemNodesItemStatusEnum::Unknown => {
                "unknown"
            }
        };
        write!(f, "{}", str_val)
    }
}
