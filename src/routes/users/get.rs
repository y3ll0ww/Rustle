use diesel::{
    expression::NonAggregate, pg::Pg, query_builder::QueryFragment, BoolExpressionMethods,
    BoxableExpression, Expression, ExpressionMethods, PgTextExpressionMethods, QueryDsl,
    RunQueryDsl,
};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::{ApiResponse, Error, Null, Success},
    auth::JwtGuard,
    cache::{self, RedisMutex},
    database::{users as database, Db},
    models::users::{PublicUser, User, UserRole, UserStatus},
    schema::users::BoxedQuery,
};

#[get("/")]
pub async fn list_all_users(
    _guard: JwtGuard,
    db: Db,
) -> Result<Success<Vec<PublicUser>>, Error<Null>> {
    let users = database::get_all_public_users(&db).await?;

    Ok(ApiResponse::success(
        format!("{} users found", users.len()),
        Some(users),
    ))
}

#[derive(Deserialize, Serialize)]
pub struct UserPagination {
    last_id: Option<Uuid>,
    users: Vec<PublicUser>,
}

#[get("/?<after>&<limit>")]
pub async fn get_all_users_paginated(
    after: Option<Uuid>,
    limit: Option<i64>,
    guard: JwtGuard,
    db: Db,
) -> Result<Success<UserPagination>, Error<Null>> {
    // Only admin can see all the users
    if guard.get_user().role != i16::from(UserRole::Admin) {
        return Err(ApiResponse::unauthorized(format!(
            "Only admin can see all users"
        )));
    }

    let users = database::get_users_after_id(&db, after, limit.unwrap_or(20)).await?;
    let users_len = users.len();
    let last_id = users.last().map(|user| user.id);

    let pagination = UserPagination { last_id, users };

    Ok(ApiResponse::success(
        format!("{users_len} users found"),
        Some(pagination),
    ))
}

#[derive(Deserialize, Serialize)]
pub struct PaginationParams {
    pub page: Option<i64>,               // default = 1
    pub limit: Option<i64>,              // default = 20
    pub search: Option<String>,          // optional text search (e.g. name)
    pub sort_by: Option<SortField>,      // e.g. "created_at", "name"
    pub sort_dir: Option<SortDirection>, // "asc" or "desc"
}

#[derive(Serialize)]
pub struct PaginatedUsers {
    records: Vec<PublicUser>,
    total: i64,
    page: i64,
    per_page: i64,
    total_pages: i64,
    has_next: bool,
    has_prev: bool,
}

#[derive(Serialize)]
pub struct PaginatedRecords<T: Serialize> {
    records: Vec<Record<T>>,
    total: i64,
    page: i64,
    limit: i64,
    total_pages: i64,
    has_next: bool,
    has_prev: bool,
}

impl<T: Serialize> PaginatedRecords<T> {
    pub fn new(total: i64, page: i64, limit: i64, total_pages: i64) -> Self {
        PaginatedRecords {
            records: Vec::new(),
            total,
            page,
            limit,
            total_pages,
            has_next: page < total_pages,
            has_prev: page > 1,
        }
    }

    pub fn add_records(mut self, data: Vec<T>) -> Self {
        let low_index = self.limit * (self.page - 1);
        let upper_bound = data.len() + low_index as usize;
        let lower_bound = ((low_index + 1) as usize).min(upper_bound);

        self.records = data
            .into_iter()
            .enumerate()
            .map(|(i, record)| Record {
                index: lower_bound + i,
                data: record,
            })
            .collect();

        self
    }
}

#[derive(Serialize)]
pub struct Record<T: Serialize> {
    index: usize,
    data: T,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    Role,
    Status,
    Username,
    DisplayName,
    Email,
    CreatedAt,
    UpdatedAt,
}

fn apply_sort<'a>(
    query: BoxedQuery<'a, Pg>,
    sort_by: &Option<SortField>,
    sort_dir: &Option<SortDirection>,
) -> BoxedQuery<'a, Pg> {
    use crate::schema::users::dsl::*;

    match sort_dir {
        Some(SortDirection::Desc) => match sort_by {
            Some(SortField::Role) => query.order(role.desc()),
            Some(SortField::Status) => query.order(status.desc()),
            Some(SortField::Username) => query.order(username.desc()),
            Some(SortField::DisplayName) => query.order(display_name.desc()),
            Some(SortField::Email) => query.order(email.desc()),
            Some(SortField::CreatedAt) => query.order(created_at.desc()),
            Some(SortField::UpdatedAt) => query.order(updated_at.desc()),
            _ => query.order(id.desc()),
        },
        // Ascending by default
        _ => match sort_by {
            Some(SortField::Role) => query.order(role.asc()),
            Some(SortField::Status) => query.order(status.asc()),
            Some(SortField::Username) => query.order(username.asc()),
            Some(SortField::DisplayName) => query.order(display_name.asc()),
            Some(SortField::Email) => query.order(email.asc()),
            Some(SortField::CreatedAt) => query.order(created_at.asc()),
            Some(SortField::UpdatedAt) => query.order(updated_at.asc()),
            _ => query.order(id.asc()),
        },
    }
}

fn build_query<'a>(
    filter_text: &str,
    filter_status: Option<i16>,
    filter_role: Option<i16>,
) -> BoxedQuery<'a, diesel::pg::Pg> {
    use crate::schema::users::dsl::*;

    let mut query = users.into_boxed::<diesel::pg::Pg>();

    if let Some(filter) = filter_status {
        query = query.filter(status.eq(filter))
    }

    if let Some(filter) = filter_role {
        query = query.filter(role.eq(filter))
    }

    // Add the following to the users table migration:
    // CREATE INDEX username_lower_idx ON users (LOWER(username));
    // CREATE INDEX email_lower_idx ON users (LOWER(email));
    // CREATE INDEX display_name_lower_idx ON users (LOWER(display_name));

    if !filter_text.is_empty() {
        let safe_search = filter_text.replace('%', "\\%").replace('_', "\\_");
        let q = format!("{safe_search}%");
        query = query.filter(
            username
                .ilike(q.clone())
                .or(email.ilike(q.clone()))
                .or(display_name.ilike(q)),
        );
    }

    query
}

//Instead of get_paginated_users, maybe browse_users or list_users_paginated — to match REST semantics more intuitively.
#[get("/browse?<status>&<role>", format = "json", data = "<params>")]
pub async fn get_paginated_users(
    status: Option<i16>,
    role: Option<i16>,
    params: Json<PaginationParams>,
    _guard: JwtGuard,
    db: Db,
) -> Result<Success<PaginatedRecords<PublicUser>>, Error<Null>> {
    // Number of the page (should be at least 1)
    let requested_page = params.page.unwrap_or(1).max(1);

    // Number of maximum results (default 20, min 1, max 100)
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let (users, total_records, page, total_pages) = db
        .run(move |conn| {
            let search_filter = params.search.as_deref().unwrap_or_default();

            // Build the query as COUNT to get the total
            let total = build_query(&search_filter, status, role)
                .count()
                .get_result::<i64>(conn)?;

            // Define the total number of pages by dividing the total by the limit and returning the upper
            // bound from the float as integer. Make sure there is at least one page.
            let total_pages = ((total as f64 / limit as f64).ceil() as i64).max(1);

            // Cap the page to total pages
            let page = requested_page.min(total_pages);

            // Calculate the offset of the search
            let offset = (page - 1) * limit;

            // Build the query again for LOAD and apply filtering
            let mut query = build_query(&search_filter, status, role);

            // Apply sorting to the query
            query = apply_sort(query, &params.sort_by, &params.sort_dir);

            // Add the offset and limit and run the query
            let users: Vec<PublicUser> = query
                .offset(offset)
                .limit(limit)
                .load::<User>(conn)
                .map(|users| users.iter().map(PublicUser::from).collect())?;

            Ok((users, total, page, total_pages))
        })
        .await
        .map_err(ApiResponse::from_error)?;

    let pagination =
        PaginatedRecords::<PublicUser>::new(total_records, page, limit, total_pages)
            .add_records(users);

    Ok(ApiResponse::success(
        format!("{} of {total_records} users shown", pagination.records.len()),
        Some(pagination),
    ))
}

#[get("/<username>")]
pub async fn get_user_by_username(
    username: &str,
    _guard: JwtGuard,
    db: Db,
) -> Result<Success<PublicUser>, Error<Null>> {
    // Only get the user from the database
    database::get_user_by_username(&db, username)
        .await
        .map(|user| {
            ApiResponse::success(
                format!("User '{username}' found"),
                Some(PublicUser::from(&user)),
            )
        })
}

#[get("/invite/get/<token>")]
pub async fn get_invited_user(
    token: &str,
    db: Db,
    redis: &State<RedisMutex>,
) -> Result<Success<Vec<String>>, Error<Null>> {
    // Get the user ID from the cache (should be a UUID at this stage)
    let user_id = cache::users::get_invite_token(redis, token).await?;

    // Get the user from the database
    let user = database::get_user_by_id(&db, user_id).await?;

    // Return not found if the user is not of status invited
    // > Returning not found avoids leaking user existence or status, preventing malicious actors
    // > from probing valid invitation tokens.
    if user.status != i16::from(UserStatus::Invited) {
        return Err(ApiResponse::not_found(format!(
            "User '{user_id}' not found",
        )));
    }

    // Return success response
    Ok(ApiResponse::success("User set in cache".to_string(), None))
}
