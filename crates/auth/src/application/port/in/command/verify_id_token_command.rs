#[derive(Debug)]
pub struct VerifyIdTokenCommand {
    pub id_token: String,
}

impl VerifyIdTokenCommand {
    pub fn new(id_token: String) -> Self {
        Self { id_token }
    }

    pub fn verify(&self) -> Result<(), String> {
        if self.id_token.trim().is_empty() {
            Err("ID token cannot be empty".to_string())
        } else {
            Ok(())
        }
    }
}
