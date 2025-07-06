use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct VerifyIdTokenRequest {
    pub id_token: String,
}
