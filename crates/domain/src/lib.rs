pub mod llm_provider;
pub mod open_ai_compatible;
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
