use rocket::State;
use uuid::Uuid;

use crate::{
    api::{Error, Null},
    cache::CACHE_TTL_ONE_HOUR,
    models::projects::{Project, ProjectWithMembers},
};

use super::RedisMutex;

pub const CACHE_PROJECT: &str = "project:";

pub fn cache_key_project(project_id: Uuid) -> String {
    format!("{CACHE_PROJECT}{project_id}")
}

pub async fn add_project_cache(
    redis: &State<RedisMutex>,
    project_with_members: &ProjectWithMembers,
) {
    let _ = redis
        .lock()
        .await
        .set_to_cache(
            &cache_key_project(project_with_members.project.id),
            project_with_members,
            CACHE_TTL_ONE_HOUR,
        )
        .await;
}

pub async fn get_project_cache(
    redis: &State<RedisMutex>,
    project_id: Uuid,
) -> Result<Option<ProjectWithMembers>, Error<Null>> {
    redis
        .lock()
        .await
        .get_from_cache(&cache_key_project(project_id))
        .await
}

pub async fn update_project_cache(
    redis: &State<RedisMutex>,
    project_id: Uuid,
    updated_project: &Project,
) {
    // Get the existing project with members from the cache
    if let Some(cached_project) = get_project_cache(redis, project_id)
        .await
        .unwrap_or_default()
    {
        let mut updated_project_with_members = cached_project;
        updated_project_with_members.project = updated_project.clone();
        add_project_cache(redis, &updated_project_with_members).await;
    }
}

pub async fn remove_project_cache(redis: &State<RedisMutex>, project_id: Uuid) {
    let _ = redis
        .lock()
        .await
        .remove_from_cache(&cache_key_project(project_id))
        .await;
}
