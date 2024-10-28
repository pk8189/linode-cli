#[derive(Clone, Debug)]
pub struct BinaryResponse {
    pub headers: http::HeaderMap,
    pub content: bytes::Bytes,
}
impl BinaryResponse {
    pub async fn new(res: reqwest::Response) -> Self {
        let headers = res.headers().clone();
        let content = res.bytes().await.unwrap_or_default();
        Self { headers, content }
    }
}
pub(crate) async fn process_json<T>(response: reqwest::Response) -> crate::SdkResult<T>
where
    T: serde::de::DeserializeOwned,
{
    let json_text = response.text().await.unwrap_or_default();
    serde_json::from_str::<T>(&json_text)
        .map_err(|e| crate::Error::DeserializeJson(e, json_text))
}
