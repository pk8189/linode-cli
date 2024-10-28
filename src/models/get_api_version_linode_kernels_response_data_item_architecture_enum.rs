#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum GetApiVersionLinodeKernelsResponseDataItemArchitectureEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "i386"))]
    #[serde(rename = "i386")]
    I386,
    #[cfg_attr(feature = "cli", value(name = "x86_64"))]
    #[serde(rename = "x86_64")]
    X8664,
}
impl std::fmt::Display for GetApiVersionLinodeKernelsResponseDataItemArchitectureEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            GetApiVersionLinodeKernelsResponseDataItemArchitectureEnum::I386 => "i386",
            GetApiVersionLinodeKernelsResponseDataItemArchitectureEnum::X8664 => "x86_64",
        };
        write!(f, "{}", str_val)
    }
}
