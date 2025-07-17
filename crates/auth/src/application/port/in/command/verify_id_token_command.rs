use crate::application::error::VerifyIdTokenCommandError;

#[derive(Debug)]
pub struct VerifyIdTokenCommand {
    pub id_token: String,
}

impl VerifyIdTokenCommand {
    pub fn new(id_token: String) -> Self {
        Self { id_token }
    }

    pub fn verify(&self) -> Result<(), VerifyIdTokenCommandError> {
        if self.id_token.trim().is_empty() {
            Err(VerifyIdTokenCommandError::EmptyToken)
        } else {
            Ok(())
        }
    }
}
