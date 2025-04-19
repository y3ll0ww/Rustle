use diesel::{Connection, ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};

use crate::{
    api::{ApiResponse, Error, Null},
    models::{
        users::{PublicUser, User},
        workspaces::{
            MemberInfo, NewWorkspace, Workspace, WorkspaceMember, WorkspaceRole,
            WorkspaceWithMembers,
        },
    },
    schema::{users, workspace_members, workspaces},
};

use super::Db;

pub async fn insert_new_workspace(
    new_workspace: NewWorkspace,
    db: &Db,
) -> Result<WorkspaceWithMembers, Error<Null>> {
    // Copy owner ID since it will go out of scope
    let owner_id = new_workspace.owner;

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
        vec![WorkspaceMember {
            workspace: workspace.id,
            member: owner_id,
            role: WorkspaceRole::Owner as i16,
        }],
        db,
    )
    .await
}

pub async fn add_members_to_workspace(
    members: Vec<WorkspaceMember>,
    db: &Db,
) -> Result<WorkspaceWithMembers, Error<Null>> {
    // Return error if there are no members to add
    if members.is_empty() {
        return Err(ApiResponse::bad_request("No members to add".to_string()));
    }

    // Get the workspace ID of the first member (they should be the same)
    let workspace_id = members[0].workspace;

    // Run database actions in a single transaction
    db.run(move |conn| {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // Insert multiple workspace members at once
            diesel::insert_into(workspace_members::table)
                .values(&members)
                .execute(conn)?;

            // Fetch workspace information and all users in the same workspace
            let results: Vec<(Workspace, WorkspaceMember, User)> = workspace_members::table
                .inner_join(workspaces::table.on(workspace_members::workspace.eq(workspaces::id)))
                .inner_join(users::table.on(workspace_members::member.eq(users::id)))
                .filter(workspace_members::workspace.eq(workspace_id))
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
        })
    })
    .await
    .map_err(ApiResponse::from_error)
}
