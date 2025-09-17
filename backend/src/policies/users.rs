use uuid::Uuid;

use crate::{
    api::{Error, Null},
    database::{self, Db},
    models::users::{PublicUser, UserRole},
};

use super::Policy;

impl Policy {
    pub async fn users_get(db: &Db, user: &PublicUser, id: Uuid) -> Result<(), Error<Null>> {
        // User is admin or self
        let base_policy = Policy::rule(user.is_admin())
            .or(user.id == id)
            .not_found("User not found");

        // If user is not admin or self; return only if the user is in a shared workspace
        // This call if run independently to prevent an unnecessary database transaction
        if base_policy.is_err() && user_is_in_same_workspace(db, user.id, id).await {
            return Ok(());
        }

        base_policy
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

async fn user_is_in_same_workspace(db: &Db, searching_user: Uuid, searched_user: Uuid) -> bool {
    match database::users::get_user_ids_in_same_workspaces(db, searching_user).await {
        Ok(users) => users.contains(&searched_user),
        Err(_) => false,
    }
}
