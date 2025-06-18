use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null},
    models::{
        projects::{NewProject, Project, ProjectMemberInfo, ProjectWithMembers},
        users::{PublicUser, User},
    },
    schema::{project_members, projects, users},
};

use super::Db;

pub async fn get_project_by_id(db: &Db, id: Uuid) -> Result<ProjectWithMembers, Error<Null>> {
    db.run(move |conn| {
        let project = projects::table
            .filter(projects::id.eq(id))
            .first::<Project>(conn)
            .map_err(ApiResponse::from_error)?;

        let members = project_members::table
            .inner_join(users::table.on(users::id.eq(project_members::member)))
            .filter(project_members::project.eq(id))
            .select((users::all_columns, project_members::role))
            .load::<(User, i16)>(conn)
            .map_err(ApiResponse::from_error)?
            .into_iter()
            .map(|(user, role)| ProjectMemberInfo {
                user: PublicUser::from(&user),
                role,
            })
            .collect();

        Ok(ProjectWithMembers { project, members })
    })
    .await
}

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

pub async fn get_projects_by_workspace_id(
    db: &Db,
    workspace: Uuid,
) -> Result<Vec<Project>, Error<Null>> {
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
