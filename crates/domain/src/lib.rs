pub mod llm_provider;
pub mod session;

#[derive(Debug)]
pub enum DialogSessionsError {
    CompletionRequestInProgress,
    RequestIdMismatch,
    NoActiveTurn,
}

pub struct CompletionResult {
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
