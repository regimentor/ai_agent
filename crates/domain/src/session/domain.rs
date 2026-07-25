use crate::DialogSessionsError;

#[cfg(test)]
mod test;

pub enum DialogRole {
    User,
    Assistant,
}

pub struct DialogMessage {
    pub role: DialogRole,
    pub content: String,
}

pub struct Turn {
    pub request_id: String,
}

enum State {
    Idle,
    Generating {
        request_id: String,
        user_input: Option<String>,
    },
}

pub struct Session {
    history: Vec<DialogMessage>,
    state: State,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Session {
    pub const fn new() -> Self {
        Session {
            history: Vec::new(),
            state: State::Idle,
        }
    }

    pub fn begin_turn(&mut self, user_input: String) -> Result<Turn, DialogSessionsError> {
        match self.state {
            State::Idle => {
                let request_id = uuid::Uuid::new_v4().to_string();
                let request_id_clone = request_id.clone();
                self.state = State::Generating {
                    request_id,
                    user_input: Some(user_input),
                };

                Ok(Turn {
                    request_id: request_id_clone,
                })
            }
            State::Generating { .. } => Err(DialogSessionsError::CompletionRequestInProgress),
        }
    }

    pub fn complete_turn(
        &mut self,
        request_id: String,
        completion: String,
    ) -> Result<(), DialogSessionsError> {
        match &self.state {
            State::Generating {
                request_id: current_request_id,
                user_input,
            } if *current_request_id == request_id => {
                if let Some(user_input) = user_input {
                    self.history.push(DialogMessage {
                        role: DialogRole::User,
                        content: user_input.clone(),
                    });
                }

                self.history.push(DialogMessage {
                    role: DialogRole::Assistant,
                    content: completion,
                });

                self.state = State::Idle;
                Ok(())
            }
            State::Generating { .. } => Err(DialogSessionsError::RequestIdMismatch),
            State::Idle => Err(DialogSessionsError::NoActiveTurn),
        }
    }

    pub fn cancel_turn(&mut self, request_id: String) -> Result<(), DialogSessionsError> {
        match &self.state {
            State::Generating {
                request_id: current_request_id,
                ..
            } if *current_request_id == request_id => {
                self.state = State::Idle;
                Ok(())
            }
            State::Generating { .. } => Err(DialogSessionsError::RequestIdMismatch),
            State::Idle => Err(DialogSessionsError::NoActiveTurn),
        }
    }

    pub fn fail_turn(&mut self, request_id: String) -> Result<(), DialogSessionsError> {
        match &self.state {
            State::Generating {
                request_id: current_request_id,
                ..
            } if *current_request_id == request_id => {
                self.state = State::Idle;
                Ok(())
            }
            State::Generating { .. } => Err(DialogSessionsError::RequestIdMismatch),
            State::Idle => Err(DialogSessionsError::NoActiveTurn),
        }
    }
}
