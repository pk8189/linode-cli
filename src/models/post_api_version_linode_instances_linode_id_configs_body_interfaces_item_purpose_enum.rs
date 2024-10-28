#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "public"))]
    #[serde(rename = "public")]
    Public,
    #[cfg_attr(feature = "cli", value(name = "vlan"))]
    #[serde(rename = "vlan")]
    Vlan,
    #[cfg_attr(feature = "cli", value(name = "vpc"))]
    #[serde(rename = "vpc")]
    Vpc,
}
impl std::fmt::Display
for PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum::Public => {
                "public"
            }
            PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum::Vlan => {
                "vlan"
            }
            PostApiVersionLinodeInstancesLinodeIdConfigsBodyInterfacesItemPurposeEnum::Vpc => {
                "vpc"
            }
        };
        write!(f, "{}", str_val)
    }
}
