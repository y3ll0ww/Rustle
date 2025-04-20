use chrono::NaiveDateTime;
use diesel::{Connection, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null},
    models::{
        users::{PublicUser, User},
        workspaces::{
            MemberInfo, NewWorkspace, Workspace, WorkspaceMember, WorkspaceRole, WorkspaceUpdate,
            WorkspaceWithMembers,
        },
    },
    schema::{users, workspace_members, workspaces},
};

use super::Db;

pub async fn get_workspaces_by_user_id(db: &Db, user: Uuid) -> Result<Vec<Workspace>, Error<Null>> {
    // Retrieve all workspaces with the user ID
    db.run(move |conn| {
        workspaces::table
            .inner_join(
                workspace_members::table.on(workspace_members::workspace.eq(workspaces::id)),
            )
            .filter(workspace_members::member.eq(user))
            .select(workspaces::all_columns)
            .load::<Workspace>(conn)
    })
    .await
    .map_err(ApiResponse::from_error)
}

pub async fn get_workspace_updated_at(db: &Db, id: Uuid) -> Result<NaiveDateTime, Error<Null>> {
    db.run(move |conn| {
        workspaces::table
            .select(workspaces::updated_at)
            .filter(workspaces::id.eq(id))
            .first::<NaiveDateTime>(conn)
            .map_err(ApiResponse::from_error)
    })
    .await
}

pub async fn get_workspace_by_id(db: &Db, id: Uuid) -> Result<WorkspaceWithMembers, Error<Null>> {
    db.run(move |conn| {
        let workspace = workspaces::table
            .filter(workspaces::id.eq(id))
            .first::<Workspace>(conn)
            .map_err(ApiResponse::from_error)?;

        let members = workspace_members::table
            .inner_join(users::table.on(users::id.eq(workspace_members::member)))
            .filter(workspace_members::workspace.eq(id))
            .select((users::all_columns, workspace_members::role))
            .load::<(User, i16)>(conn)
            .map_err(ApiResponse::from_error)?
            .into_iter()
            .map(|(user, role)| MemberInfo {
                user: PublicUser::from(&user),
                role,
            })
            .collect();

        Ok(WorkspaceWithMembers { workspace, members })
    })
    .await
}

pub async fn insert_new_workspace(
    db: &Db,
    new_workspace: NewWorkspace,
) -> Result<WorkspaceWithMembers, Error<Null>> {
    // Insert and return a new workspace
    let workspace = db
        .run(move |conn| {
            diesel::insert_into(workspaces::table)
                .values(new_workspace)
                .get_result::<Workspace>(conn)
        })
        .await
        .map_err(ApiResponse::from_error)?;

    // Add the owner to the workspace
    add_members_to_workspace(
        db,
        vec![WorkspaceMember {
            workspace: workspace.id,
            member: workspace.owner,
            role: i16::from(WorkspaceRole::Owner),
        }],
    )
    .await
}

pub async fn add_members_to_workspace(
    db: &Db,
    members: Vec<WorkspaceMember>,
) -> Result<WorkspaceWithMembers, Error<Null>> {
    // Get the workspace ID of the first member (they should be the same)
    let workspace_id = members[0].workspace;

    // Run database actions in a single transaction
    db.run(move |conn| {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // Insert multiple workspace members at once
            diesel::insert_into(workspace_members::table)
                .values(&members)
                .execute(conn)?;

            fetch_workspace_with_members(workspace_id, conn)
        })
    })
    .await
    .map_err(ApiResponse::from_error)
}

pub async fn remove_member_from_workspace(
    db: &Db,
    workspace: Uuid,
    member: Uuid,
) -> Result<WorkspaceWithMembers, Error<Null>> {
    db.run(move |conn| {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let removed_records = diesel::delete(
                workspace_members::table
                    .filter(workspace_members::workspace.eq(workspace))
                    .filter(workspace_members::member.eq(member)),
            )
            .execute(conn)?;

            if removed_records == 0 {
                return Err(diesel::result::Error::NotFound);
            }

            fetch_workspace_with_members(workspace, conn)
        })
    })
    .await
    .map_err(ApiResponse::from_error)
}

pub async fn update_workspace_information(
    db: &Db,
    id: Uuid,
    update: WorkspaceUpdate,
) -> Result<Workspace, Error<Null>> {
    db.run(move |conn| {
        // Update the workspace table with information from the update
        diesel::update(workspaces::table.filter(workspaces::id.eq(id)))
            .set(update)
            .get_result::<Workspace>(conn)
            .map_err(ApiResponse::from_error)
    })
    .await
}

pub async fn remove_workspace(db: &Db, id: Uuid) -> Result<Workspace, Error<Null>> {
    db.run(move |conn| {
        diesel::delete(workspaces::table.filter(workspaces::id.eq(id)))
            .get_result::<Workspace>(conn)
    })
    .await
    .map_err(ApiResponse::from_error)
}

fn fetch_workspace_with_members(
    id: Uuid,
    conn: &mut PgConnection,
) -> Result<WorkspaceWithMembers, diesel::result::Error> {
    // Fetch workspace information and all users in the same workspace
    let results: Vec<(Workspace, WorkspaceMember, User)> = workspace_members::table
        .inner_join(workspaces::table.on(workspace_members::workspace.eq(workspaces::id)))
        .inner_join(users::table.on(workspace_members::member.eq(users::id)))
        .filter(workspace_members::workspace.eq(id))
        .select((
            workspaces::all_columns,
            workspace_members::all_columns,
            users::all_columns,
        ))
        .load::<(Workspace, WorkspaceMember, User)>(conn)?;

    // Return error if there are no results
    if results.is_empty() {
        return Err(diesel::result::Error::NotFound);
    }

    // Get the workspace object from the first result (again, they're assumed all to be the same)
    let workspace = results[0].0.clone();

    // Define the members of the workspace based on the users from the result
    let members = results
        .iter()
        .map(|(_, membership, user)| MemberInfo {
            user: PublicUser::from(user),
            role: membership.role,
        })
        .collect();

    // Return the workspace information containing all public member information
    Ok(WorkspaceWithMembers { workspace, members })
}
