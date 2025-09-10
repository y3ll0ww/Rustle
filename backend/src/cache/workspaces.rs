use rocket::State;
use uuid::Uuid;

use crate::{
    api::{Error, Null},
    cache::CACHE_TTL_ONE_HOUR,
    models::workspaces::{Workspace, WorkspaceWithMembers},
};

use super::RedisMutex;

pub const CACHE_WORKSPACE: &str = "workspace:";

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
    updated_workspace: &Workspace,
) {
    // Get the existing workspace with members from the cache
    if let Some(cached_workspace) = get_workspace_cache(redis, workspace_id)
        .await
        .unwrap_or_default()
    {
        let mut updated_workspace_with_members = cached_workspace;
        updated_workspace_with_members.workspace = updated_workspace.clone();
        add_workspace_cache(redis, &updated_workspace_with_members).await;
    }
}

pub async fn remove_workspace_cache(redis: &State<RedisMutex>, workspace_id: Uuid) {
    let _ = redis
        .lock()
        .await
        .remove_from_cache(&cache_key_workspace(workspace_id))
        .await;
}
