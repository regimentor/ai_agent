use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

use crate::session::domain::DialogMessage;

pub enum Error {
    NetworkError(String),
    InvalidResponse(String),
    ProtocolError(String),
    Other(String),
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
