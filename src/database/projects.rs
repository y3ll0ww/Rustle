use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null},
    models::projects::{NewProject, Project, ProjectWithMembers},
    schema::{project_members, projects},
};

use super::Db;

pub async fn get_projects_by_user_id(db: &Db, user: Uuid) -> Result<Vec<Project>, Error<Null>> {
    // Retrieve all workspaces with the user ID
    db.run(move |conn| {
        projects::table
            .inner_join(project_members::table.on(project_members::project.eq(projects::id)))
            .filter(project_members::member.eq(user))
            .select(projects::all_columns)
            .load::<Project>(conn)
    })
    .await
    .map_err(ApiResponse::from_error)
}

pub async fn get_projects_by_workspace_id(db: &Db, workspace: Uuid) -> Result<Vec<Project>, Error<Null>> {
    // Retrieve all workspaces with the user ID
    db.run(move |conn| {
        projects::table
            .filter(projects::workspace.eq(workspace))
            .select(projects::all_columns)
            .load::<Project>(conn)
    })
    .await
    .map_err(ApiResponse::from_error)
}

pub async fn insert_new_project(
    db: &Db,
    workspace: Uuid,
    new_project: NewProject,
) -> Result<ProjectWithMembers, Error<Null>> {
    let project = db
        .run(move |conn| {
            diesel::insert_into(projects::table)
                .values((
                    projects::workspace.eq(workspace),
                    projects::name.eq(new_project.name),
                    projects::description.eq(new_project.description),
                ))
                .get_result::<Project>(conn)
        })
        .await
        .map_err(ApiResponse::from_error)?;

    Ok(ProjectWithMembers {
        project,
        members: Vec::new(),
    })
}
