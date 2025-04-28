use rocket::State;
use uuid::Uuid;

use crate::{
    api::{Error, Null},
    models::workspaces::{MemberInfo, WorkspaceUpdate, WorkspaceWithMembers},
};

use super::RedisMutex;

pub const CACHE_WORKSPACE: &str = "workspace:";
pub const CACHE_TTL_ONE_HOUR: Option<u64> = Some(3600); // One hour

pub fn cache_key_workspace(workspace_id: Uuid) -> String {
    format!("{CACHE_WORKSPACE}{workspace_id}")
}

pub async fn add_workspace_cache(
    redis: &State<RedisMutex>,
    workspace_with_members: &WorkspaceWithMembers,
) {
    let _ = redis
        .lock()
        .await
        .set_to_cache(
            &cache_key_workspace(workspace_with_members.workspace.id),
            workspace_with_members,
            CACHE_TTL_ONE_HOUR,
        )
        .await;
}

pub async fn get_workspace_cache(
    redis: &State<RedisMutex>,
    workspace_id: Uuid,
) -> Result<Option<WorkspaceWithMembers>, Error<Null>> {
    redis
        .lock()
        .await
        .get_from_cache(&cache_key_workspace(workspace_id))
        .await
}

pub async fn update_workspace_cache(
    redis: &State<RedisMutex>,
    workspace_id: Uuid,
    update: Option<WorkspaceUpdate>,
    members: Option<Vec<MemberInfo>>,
) {
    // Get the existing workspace cache
    if let Some(cached_workspace) = get_workspace_cache(redis, workspace_id)
        .await
        .unwrap_or_default()
    {
        // Define the new workspace for in the cache
        let workspace_with_members = WorkspaceWithMembers {
            // Apply the update or use the information from the cache
            workspace: update
                .map(|workspace_update| {
                    let mut workspace = cached_workspace.workspace.clone();
                    workspace.update(workspace_update);
                    workspace
                })
                .unwrap_or(cached_workspace.workspace),
            // Apply the members vector or use the information from the cache
            members: members.unwrap_or(cached_workspace.members),
        };

        // Set the cache with the updated workspace information
        let _ = redis
            .lock()
            .await
            .set_to_cache(
                &cache_key_workspace(workspace_id),
                &workspace_with_members,
                CACHE_TTL_ONE_HOUR,
            )
            .await;
    }
}

pub async fn remove_workspace_cache(redis: &State<RedisMutex>, workspace_id: Uuid) {
    let _ = redis
        .lock()
        .await
        .remove_from_cache(&cache_key_workspace(workspace_id))
        .await;
}
