use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    pub id_token: String,
}
