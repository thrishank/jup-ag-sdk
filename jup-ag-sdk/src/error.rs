use reqwest::{Response, StatusCode};

#[derive(Debug, thiserror::Error)]
pub enum JupiterClientError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Invalid header value: {0}")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error("API returned error: {0}, Status Code: {1}")]
    ApiError(String, StatusCode),

    #[error("Failed to deserialize response: {0}")]
    DeserializationError(String),
}

pub async fn handle_response(response: Response) -> Result<Response, JupiterClientError> {
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unable to get error details".to_string());
        return Err(JupiterClientError::ApiError(error_text, status));
    }
    Ok(response)
}
