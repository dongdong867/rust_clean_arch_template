use serde::Deserialize;

use crate::AuthenticatedUser;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AuthenticatedUserDto {
    pub id: String,
    pub email: String,
    pub name: String,
}

impl From<AuthenticatedUserDto> for AuthenticatedUser {
    fn from(val: AuthenticatedUserDto) -> Self {
        AuthenticatedUser {
            id: val.id,
            email: val.email,
            name: val.name,
        }
    }
}
