use rocket::State;
use uuid::Uuid;

use crate::{
    api::{Error, Null},
    cache::{self, RedisMutex},
    database::{self, Db},
    models::{projects::ProjectWithMembers, workspaces::WorkspaceWithMembers},
    routes::workspaces::get_workspace_with_members,
};

mod delete;
mod get;
mod post;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get::get_projects_of_current_user, // GET:     /projects
        post::create_new_project_by_form,  // POST:    /projects/<workspace>/new
        get::get_project_by_id,            // GET:     /projects/<id>
        post::add_members_to_project,      // POST:    /projects/<id>/add-members
        delete::delete_project_by_id,      // DELETE:  /projects/<id>/delete
    ]
}

/// Get both the [`ProjectWithMembers`] and the [`WorkspaceWithMembers`] information, of which the
/// project is a part.
///
/// * If calls [`get_project_with_members`] to get all necessary information of the project, then
///   extracts the [`Project`](crate::models::projects::Project) and from it the
///   [`workspace ID`](crate::models::projects::Project::workspace).
/// * Using this ID, it calls [`get_workspace_with_members`] to get all necessary information of
///   the workspace.
/// * Lastly, it returns both objects in a tuple for further handling.
pub async fn get_workspace_and_project(
    project_id: Uuid,
    db: &Db,
    redis: &State<RedisMutex>,
) -> Result<(WorkspaceWithMembers, ProjectWithMembers), Error<Null>> {
    // Get the project with members
    let project_with_members = get_project_with_members(project_id, db, redis).await?;

    // Get the workspace information with its members
    let workspace_with_members =
        get_workspace_with_members(project_with_members.project.workspace, db, redis).await?;

    // Return the objects together
    Ok((workspace_with_members, project_with_members))
}

async fn get_project_with_members(
    id: Uuid,
    db: &Db,
    redis: &State<RedisMutex>,
) -> Result<ProjectWithMembers, Error<Null>> {
    // Attempt getting the project from the cache
    Ok(match cache::projects::get_project_cache(redis, id).await? {
        Some(cached_project) => cached_project,
        None => {
            // Get the project with members from the database
            let project_from_database = database::projects::get_project_by_id(db, id).await?;

            // Add the project with members to the cache
            cache::projects::add_project_cache(redis, &project_from_database).await;

            // Return a fresh project with members from the database
            project_from_database
        }
    })
}
