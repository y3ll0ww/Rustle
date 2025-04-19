use rocket::State;
use uuid::Uuid;

use crate::{
    forms::workspace::UpdateWorkspaceForm,
    models::workspaces::{MemberInfo, WorkspaceWithMembers},
};

use super::RedisMutex;

pub const CACHE_WORKSPACE: &str = "workspace:";
pub const CACHE_TTL_ONE_HOUR: Option<u64> = Some(3600); // One hour

pub fn workspace_cache_key(workspace_id: Uuid) -> String {
    format!("{CACHE_WORKSPACE}{workspace_id}")
}

pub async fn set_workspace_cache(
    redis: &State<RedisMutex>,
    workspace_with_members: &WorkspaceWithMembers,
) {
    let _ = redis
        .lock()
        .await
        .set_to_cache(
            &workspace_cache_key(workspace_with_members.workspace.id),
            &workspace_with_members,
            CACHE_TTL_ONE_HOUR,
        )
        .await;
}

pub async fn update_workspace_cache(
    redis: &State<RedisMutex>,
    workspace_id: Uuid,
    form: Option<UpdateWorkspaceForm>,
    members: Option<Vec<MemberInfo>>,
) -> bool {
    // Get the existing workspace cache
    if let Some(workspace_with_members) = redis
        .lock()
        .await
        .get_from_cache::<WorkspaceWithMembers>(&workspace_cache_key(workspace_id))
        .await
        .unwrap_or(None)
    {
        let workspace_with_members = WorkspaceWithMembers {
            workspace: form
                .map(|update_workspace_form| {
                    let mut workspace = workspace_with_members.workspace.clone();
                    workspace.update(update_workspace_form);
                    workspace
                })
                .unwrap_or(workspace_with_members.workspace),
            members: members.unwrap_or(workspace_with_members.members),
        };

        // Set the cache with the updated workspace information
        let _ = redis
            .lock()
            .await
            .set_to_cache(
                &workspace_cache_key(workspace_id),
                &workspace_with_members,
                CACHE_TTL_ONE_HOUR,
            )
            .await;

        return true;
    };

    false
}
