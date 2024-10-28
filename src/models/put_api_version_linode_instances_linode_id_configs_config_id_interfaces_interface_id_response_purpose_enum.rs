#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum {
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
for PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum::Public => {
                "public"
            }
            PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum::Vlan => {
                "vlan"
            }
            PutApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum::Vpc => {
                "vpc"
            }
        };
        write!(f, "{}", str_val)
    }
}
