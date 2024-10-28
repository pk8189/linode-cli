#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum PutApiVersionAccountUsersUsernameGrantsResponseLinodeItemPermissionsEnum {
    #[default]
    #[cfg_attr(feature = "cli", value(name = "read_only"))]
    #[serde(rename = "read_only")]
    ReadOnly,
    #[cfg_attr(feature = "cli", value(name = "read_write"))]
    #[serde(rename = "read_write")]
    ReadWrite,
}
impl std::fmt::Display
for PutApiVersionAccountUsersUsernameGrantsResponseLinodeItemPermissionsEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_val = match self {
            PutApiVersionAccountUsersUsernameGrantsResponseLinodeItemPermissionsEnum::ReadOnly => {
                "read_only"
            }
            PutApiVersionAccountUsersUsernameGrantsResponseLinodeItemPermissionsEnum::ReadWrite => {
                "read_write"
            }
        };
        write!(f, "{}", str_val)
    }
}
