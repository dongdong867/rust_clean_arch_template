use serde::Deserialize;

use crate::AuthenticatedUser;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AuthenticatedUserDto {
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
