#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error occured: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error occured making the request: {0}")]
    Request(#[from] reqwest::Error),
    #[error(
        "Error deserializing JSON into expected type: {}",
        serde_json::to_string_pretty(
            &serde_json::from_str::<serde_json::Value>(.1).unwrap_or_default()
        ).unwrap_or_else(|_|.1.to_string())
    )]
    DeserializeJson(serde_json::Error, String),
    #[error("Api returned an error status: {}", .0.status)]
    Api(ApiError),
    #[error("Api returned an unexpected content type")]
    ContentType(ApiError),
}
#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
    pub content: bytes::Bytes,
}
impl ApiError {
    pub async fn new(res: reqwest::Response) -> Self {
        let status = res.status();
        let headers = res.headers().clone();
        let content = res.bytes().await.unwrap_or_default();
        Self { status, headers, content }
    }
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.content)
    }
}
