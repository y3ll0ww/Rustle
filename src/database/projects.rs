use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null},
    models::projects::Project,
    schema::{project_members, projects},
};

use super::Db;

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
