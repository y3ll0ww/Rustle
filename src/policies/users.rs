use uuid::Uuid;

use crate::{
    api::{Error, Null},
    models::users::{PublicUser, UserRole},
};

use super::Policy;

impl Policy {
    pub fn users_get(user: &PublicUser, id: Uuid) -> Result<(), Error<Null>> {
        // User is admin
        Policy::rule(user.is_admin())
            // User is self
            .or(user.id == id)
            .not_found("User not found")
    }

    /// Policy for updating basic information of a user
    pub fn users_update_info(user: &PublicUser, id: Uuid) -> Result<(), Error<Null>> {
        // User is at least Manager
        Policy::rule(user.is_at_least(UserRole::Manager))
            // Or is updating self
            .or(user.id == id)
            .unauthorized("No permission to update user")
    }

    /// Policy for updating the status field of a user
    pub fn users_set_status(user: &PublicUser) -> Result<(), Error<Null>> {
        // User is at least Manager
        Policy::rule(user.is_at_least(UserRole::Manager))
            .unauthorized(&format!("Must be at least {}", UserRole::Manager))
    }

    /// Policy for updating the role of a user
    pub fn users_set_role(user: &PublicUser, role: i16) -> Result<(), Error<Null>> {
        // User is at least Manager
        Policy::rule(user.is_at_least(UserRole::Manager))
            // And cannot set a role higher as self
            .and(role <= user.role)
            .unauthorized("No permission to set user role")
    }

    /// Policy for deleting a user from the database
    pub fn users_delete(user: &PublicUser, id: Uuid) -> Result<(), Error<Null>> {
        // User is admin
        Policy::rule(user.is_admin())
            // User is self
            .or(user.id == id)
            .unauthorized("No permission to delete user")
    }
}
