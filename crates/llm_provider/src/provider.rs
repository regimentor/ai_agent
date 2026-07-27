pub struct Provider {
    base_url: String,
    model: String,
}

impl Provider {
    pub const fn new(base_url: String, model: String) -> Self {
        Provider { base_url, model }
    }

    async fn completition_request(
        &self,
        messages: &[crate::completion_request::Message],
        cancellation_token: tokio_util::sync::CancellationToken,
    ) -> Result<String, crate::error::Error> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let request_body = crate::completion_request::CompletionRequest {
            model: self.model.clone(),
            messages: messages.to_vec(),
        };

        let client = reqwest::Client::new();
        let response = client.post(&url).json(&request_body).send();

        let response = tokio::select! {
            res = response => {
                match res {
                    Ok(resp) => resp,
                    Err(e) => return Err(crate::error::Error::Network(e.to_string())),
                }
            },
            _ = cancellation_token.cancelled() => {
                return Err(crate::error::Error::Network("Request cancelled".to_string()));
            }
        };

        let status = response.status();
        if !status.is_success() {
            return Err(crate::error::Error::Network(format!(
                "Request failed with status: {}",
                status
            )));
        }

        let parsed_response: crate::completion_response::ChatCompletionResponse = response
            .json()
            .await
            .map_err(|e| crate::error::Error::InvalidResponse(e.to_string()))?;

        let content = parsed_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| {
                crate::error::Error::InvalidResponse("No choices in response".to_string())
            })?;

        Ok(content)
    }
}

#[async_trait::async_trait]
impl domain::llm_provider::Provider for Provider {
    type CompletionResult = String;

    async fn completion(
        &self,
        history: &[domain::session::DialogMessage],
        user_input: &str,
        cancellation_token: tokio_util::sync::CancellationToken,
    ) -> Result<Self::CompletionResult, domain::llm_provider::ProviderError> {
        // Convert the history and user_input into the format expected by the LLM API
        let mut messages: Vec<crate::completion_request::Message> = history
            .iter()
            .map(|msg| {
                let role = match msg.role {
                    domain::session::DialogRole::User => crate::completion_request::Role::User,
                    domain::session::DialogRole::Assistant => {
                        crate::completion_request::Role::Assistant
                    }
                };

                crate::completion_request::Message {
                    role,
                    content: msg.content.clone(),
                }
            })
            .collect();
        messages.insert(
            0,
            crate::completion_request::Message {
                role: crate::completion_request::Role::System,
                content: "You are a helpful assistant.".to_string(),
            },
        );
        messages.push(crate::completion_request::Message {
            role: crate::completion_request::Role::User,
            content: user_input.to_string(),
        });

        // Call the completition_request method to get the completion result
        match self
            .completition_request(&messages, cancellation_token)
            .await
        {
            Ok(result) => Ok(result),
            Err(e) => {
                eprintln!("Error during completion request: {e}");
                Err(domain::llm_provider::ProviderError::Other(format!(
                    "Failed to get completion"
                )))
            }
        }
    }
}
