pub const TEAM_CACHE_KEY: &str = "team:";
pub const TEAM_CACHE_TTL: Option<u64> = Some(3600); // One hour

pub fn team_cache_key(team_id: &str) -> String {
    format!("{TEAM_CACHE_KEY}{team_id}")
}
