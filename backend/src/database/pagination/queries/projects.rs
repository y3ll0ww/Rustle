use diesel::{
    dsl::exists, pg::Pg, BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods,
    QueryDsl,
};
use uuid::Uuid;

use crate::{
    database::pagination::sort::{ProjectField, SortDirection},
    schema::projects::BoxedQuery as ProjectQuery,
};

pub fn sort<'a>(
    query: ProjectQuery<'a, Pg>,
    sort_by: &Option<ProjectField>,
    sort_dir: &Option<SortDirection>,
) -> ProjectQuery<'a, Pg> {
    use crate::schema::projects::dsl::*;

    match sort_dir {
        Some(SortDirection::Desc) => match sort_by {
            Some(ProjectField::Name) => query.order(name.desc()),
            Some(ProjectField::MemberCount) => query.order(member_count.desc()),
            Some(ProjectField::CreatedAt) => query.order(created_at.desc()),
            Some(ProjectField::UpdatedAt) => query.order(updated_at.desc()),
            _ => query.order(id.desc()),
        },
        // Ascending by default
        _ => match sort_by {
            Some(ProjectField::Name) => query.order(name.asc()),
            Some(ProjectField::MemberCount) => query.order(member_count.asc()),
            Some(ProjectField::CreatedAt) => query.order(created_at.asc()),
            Some(ProjectField::UpdatedAt) => query.order(updated_at.asc()),
            _ => query.order(id.asc()),
        },
    }
}

pub fn build<'a>(
    filter_search: &str,
    workspace: Option<Uuid>,
    user: Option<Uuid>,
) -> ProjectQuery<'a, diesel::pg::Pg> {
    use crate::schema::{
        project_members::dsl as project_members_dsl,
        projects::{self, dsl as projects_dsl},
    };

    // Declare the query (mutable)
    let mut query = projects_dsl::projects.into_boxed::<diesel::pg::Pg>();

    // If workspace ID is provided add it as a filter
    if let Some(workspace_id) = workspace {
        query = query.filter(projects::workspace.eq(workspace_id));
    }

    // If user ID is provided add it as a filter
    if let Some(user_id) = user {
        query = query.filter(exists(
            project_members_dsl::project_members
                .filter(project_members_dsl::project.eq(projects::id))
                .filter(project_members_dsl::member.eq(user_id)),
        ));
    }

    // Add the search filter
    if !filter_search.is_empty() {
        // Add excape characters for unsafe characters
        let safe_search = format!(
            "%{}%",
            filter_search.replace('%', "\\%").replace('_', "\\_")
        );

        // Apply the search filter on the query
        query = query.filter(
            projects::name
                .ilike(safe_search.clone())
                .or(projects::description.ilike(safe_search.clone())),
        );
    }

    // Remove self from the list
    query
}
