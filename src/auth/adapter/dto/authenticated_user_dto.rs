use serde::Deserialize;

use crate::auth::AuthenticatedUser;

#[derive(Debug, Deserialize)]
pub(crate) struct AuthenticatedUserDto {
    pub id: String,
    pub email: String,
    pub name: String,
}

impl Into<AuthenticatedUser> for AuthenticatedUserDto {
    fn into(self) -> AuthenticatedUser {
        AuthenticatedUser {
            id: self.id,
            email: self.email,
            name: self.name,
        }
    }
}
