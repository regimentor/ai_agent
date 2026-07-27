use async_trait::async_trait;
use derive_more::Display;
use tokio_util::sync::CancellationToken;

use crate::session::domain::DialogMessage;

#[derive(Debug, Display)]
pub enum Error {
    #[display("Network error: {_0}")]
    NetworkError(String),
    #[display("Invalid response: {_0}")]
    InvalidResponse(String),
    #[display("Protocol error: {_0}")]
    ProtocolError(String),
    #[display("Other error: {_0}")]
    Other(String),
    #[display("Request timed out")]
    Timeout,
}

#[async_trait]
pub trait Provider {
    type CompletionResult;

    async fn completion(
        &self,
        history: &[DialogMessage],
        user_input: &str,
        cancellation_token: CancellationToken,
    ) -> Result<Self::CompletionResult, Error>;
}
