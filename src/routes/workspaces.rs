use rocket::State;
use uuid::Uuid;

use crate::{
    api::{Error, Null},
    cache::{self, RedisMutex},
    database::{self, Db},
    models::workspaces::WorkspaceWithMembers,
};

mod delete;
mod get;
mod post;
mod put;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get::get_workspaces_of_current_user,  // GET:     /workspaces
        post::create_new_workspace_by_form,   // POST:    /workspaces/new
        get::get_workspace_by_id,             // GET:     /workspaces/<id>
        post::add_members_to_workspace,       // POST:    /workspaces/<id>/add-members
        delete::delete_workspace_by_id,       // DELETE:  /workspaces/<id>/delete
        post::invite_new_users_to_workspace,  // POST:    /workspaces/<id>/invite
        get::get_projects_by_id,              // GET:     /workspaces/<id>/projects
        put::update_workspace,                // PUT:     /workspaces/<id>/update
        post::reinvite_user_by_id,            // POST:    /workspaces/<id>/re-invite/<member>")]
        delete::remove_member_from_workspace, // DELETE:  /workspaces/<id>/remove-member/<member>
    ]
}

/// Public function to be used by other modules
pub async fn get_workspace_with_members(
    id: Uuid,
    db: &Db,
    redis: &State<RedisMutex>,
) -> Result<WorkspaceWithMembers, Error<Null>> {
    Ok(
        match cache::workspaces::get_workspace_cache(redis, id).await? {
            Some(cached_workspace) => cached_workspace,
            None => {
                // Get the workspace with members from the database
                let workspace_from_database =
                    database::workspaces::get_workspace_by_id(db, id).await?;

                // Add the workspace with members to the cache
                cache::workspaces::add_workspace_cache(redis, &workspace_from_database).await;

                // Return a fresh workspace with members from the database
                workspace_from_database
            }
        },
    )
}
