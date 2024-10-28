#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum {
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
for GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum::Public => {
                "public"
            }
            GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum::Vlan => {
                "vlan"
            }
            GetApiVersionLinodeInstancesLinodeIdConfigsConfigIdInterfacesInterfaceIdResponsePurposeEnum::Vpc => {
                "vpc"
            }
        };
        write!(f, "{}", str_val)
    }
}
