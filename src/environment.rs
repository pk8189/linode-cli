use std::fmt::Display;
#[derive(Debug, Default, Clone)]
pub enum Environment {
    #[default]
    Default,
    MockServer,
}
impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Default => write!(f, "https://api.linode.com"),
            Environment::MockServer => {
                write!(f, "http://127.0.0.1:8082/v1/mock/local/linode/0.1.2")
            }
        }
    }
}
#[derive(Clone, Debug)]
pub(crate) enum BaseUrl {
    Env(crate::environment::Environment),
    Custom(String),
}
impl Default for BaseUrl {
    fn default() -> Self {
        BaseUrl::Env(crate::environment::Environment::default())
    }
}
impl std::fmt::Display for BaseUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Env(e) => write!(f, "{e}"),
            Self::Custom(url) => write!(f, "{url}"),
        }
    }
}
