use rocket::State;
use uuid::Uuid;

use crate::{
    forms::teams::UpdateTeamForm,
    models::teams::{TeamMemberInfo, TeamWithMembers},
};

use super::RedisMutex;

pub const TEAM_CACHE_KEY: &str = "team:";
pub const TEAM_CACHE_TTL: Option<u64> = Some(3600); // One hour

pub fn team_cache_key(team_id: &Uuid) -> String {
    format!("{TEAM_CACHE_KEY}{team_id}")
}

pub async fn set_team_cache(redis: &State<RedisMutex>, team_id: &Uuid, team: &TeamWithMembers) {
    let _ = redis
        .lock()
        .await
        .set_to_cache(&team_cache_key(team_id), &team, TEAM_CACHE_TTL)
        .await;
}

pub async fn update_team_cache(
    redis: &State<RedisMutex>,
    team_id: &Uuid,
    form: Option<UpdateTeamForm>,
    members: Option<Vec<TeamMemberInfo>>,
) -> bool {
    // Get the existing team cache
    if let Some(team_from_cache) = redis
        .lock()
        .await
        .get_from_cache::<TeamWithMembers>(&team_cache_key(team_id))
        .await
        .unwrap_or(None)
    {
        let team_with_members = TeamWithMembers {
            team: form
                .map(|update_team_form| {
                    let mut team = team_from_cache.team.clone();
                    team.update(update_team_form);
                    team
                })
                .unwrap_or(team_from_cache.team),
            members: members.unwrap_or(team_from_cache.members),
        };

        // Set the cache with the updated team information
        let _ = redis
            .lock()
            .await
            .set_to_cache(
                &team_cache_key(team_id),
                &team_with_members,
                TEAM_CACHE_TTL,
            )
            .await;

        return true;
    };

    false
}
