use serde::Serialize;

#[derive(Serialize)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}
