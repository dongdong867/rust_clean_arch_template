use auth::domain::AuthenticatedUser;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    pub id: String,
    pub email: String,
    pub name: String,
}

impl From<AuthenticatedUser> for VerifyResponse {
    fn from(user: AuthenticatedUser) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
        }
    }
}
