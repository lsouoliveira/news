use reqwest::header::HeaderMap;
use crate::providers::core::error::Error;
use crate::providers::core::response::Response;

pub struct Client {
    base_url: String,
    headers: HeaderMap,
}

impl Client {
    pub fn new(base_url: String, headers: HeaderMap) -> Client {
        Client { base_url, headers }
    }

    pub async fn get(&self, path: &str) -> Result<Response, Error> {
        self.perform_request(reqwest::Method::GET, &path).await
    }

    async fn perform_request(&self, method: reqwest::Method, path: &str) -> Result<Response, Error> {
        let url = format!("{}/{}", self.base_url, path);
        let client = reqwest::Client::new();
        let response = client.request(method, &url)
            .headers(self.headers.clone())
            .send()
            .await
            .map_err(|_| Error::build_client_error("Failed to send request"))?;

        let is_success = response.status().is_success();
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_else(|_| "".to_string());

        if is_success {
            Ok(Response::new(status, body))
        } else {
            Err(Error::build_request_error(status, body))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        let mut server = mockito::Server::new_async().await;

        let base_url = server.url();

        let client = Client::new(base_url.to_string(), Default::default());
        let _m = server.mock("GET", "/")
            .with_status(200)
            .with_body("Hello, world!")
            .create_async()
            .await;

        let response = client.get("").await.unwrap();

        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), "Hello, world!");
    } }
