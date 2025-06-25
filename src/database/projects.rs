use diesel::{Connection, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null},
    models::{
        projects::{NewProject, Project, ProjectMember, ProjectWithMembers},
        users::{PublicUser, User},
        MemberInfo,
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
            .map(|(user, role)| MemberInfo {
                user: PublicUser::from(&user),
                role,
            })
            .collect();

        Ok(ProjectWithMembers { project, members })
    })
    .await
}

pub async fn get_projects_by_user_id(db: &Db, user: Uuid) -> Result<Vec<Project>, Error<Null>> {
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

pub async fn remove_project(db: &Db, id: Uuid) -> Result<Project, Error<Null>> {
    db.run(move |conn| {
        diesel::delete(projects::table.filter(projects::id.eq(id))).get_result::<Project>(conn)
    })
    .await
    .map_err(ApiResponse::from_error)
}

pub async fn add_members_to_project(
    db: &Db,
    members: Vec<ProjectMember>,
) -> Result<ProjectWithMembers, Error<Null>> {
    // Get the project ID of the first member (they should be the same)
    let project_id = members[0].project;

    // Run database actions in a single transaction
    db.run(move |conn| {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // Insert multiple workspace members at once
            diesel::insert_into(project_members::table)
                .values(&members)
                .execute(conn)?;

            fetch_project_with_members(project_id, conn)
        })
    })
    .await
    .map_err(ApiResponse::from_error)
}

pub async fn remove_member_from_project(
    db: &Db,
    project: Uuid,
    member: Uuid,
) -> Result<ProjectWithMembers, Error<Null>> {
    db.run(move |conn| {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let removed_records = diesel::delete(
                project_members::table
                    .filter(project_members::project.eq(project))
                    .filter(project_members::member.eq(member)),
            )
            .execute(conn)?;

            if removed_records == 0 {
                return Err(diesel::result::Error::NotFound);
            }

            fetch_project_with_members(project, conn)
        })
    })
    .await
    .map_err(ApiResponse::from_error)
}

fn fetch_project_with_members(
    id: Uuid,
    conn: &mut PgConnection,
) -> Result<ProjectWithMembers, diesel::result::Error> {
    // Fetch the project first
    let project = projects::table
        .filter(projects::id.eq(id))
        .first::<Project>(conn)?;

    // Then fetch members for that project
    let member_results: Vec<(ProjectMember, User)> = project_members::table
        .inner_join(users::table.on(project_members::member.eq(users::id)))
        .filter(project_members::project.eq(id))
        .select((project_members::all_columns, users::all_columns))
        .load(conn)?;

    // Build members list
    let members: Vec<MemberInfo> = member_results
        .into_iter()
        .map(|(membership, user)| MemberInfo {
            user: PublicUser::from(&user),
            role: membership.role,
        })
        .collect();

    // Return the assembled result
    Ok(ProjectWithMembers { project, members })
}
