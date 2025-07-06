use chrono::Utc;
use serde::Serialize;

use crate::domain::AuthenticatedUser;

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct VerifyIdTokenResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub timestamp: i64,
}

impl From<AuthenticatedUser> for VerifyIdTokenResponse {
    fn from(user: AuthenticatedUser) -> Self {
        VerifyIdTokenResponse {
            id: user.id,
            name: user.name,
            email: user.email,
            timestamp: Utc::now().timestamp(),
        }
    }
}
