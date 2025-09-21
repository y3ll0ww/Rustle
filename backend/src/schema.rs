// @generated automatically by Diesel CLI.

diesel::table! {
    project_members (project, member) {
        project -> Uuid,
        member -> Uuid,
        role -> Int2,
    }
}

diesel::table! {
    projects (id) {
        id -> Uuid,
        workspace -> Uuid,
        #[max_length = 40]
        name -> Varchar,
        description -> Nullable<Text>,
        member_count -> Int4,
        image_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 40]
        username -> Varchar,
        #[max_length = 20]
        first_name -> Varchar,
        #[max_length = 20]
        last_name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        role -> Int2,
        status -> Int2,
        #[max_length = 20]
        job_title -> Nullable<Varchar>,
        password -> Text,
        bio -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    workspace_members (workspace, member) {
        workspace -> Uuid,
        member -> Uuid,
        role -> Int2,
    }
}

diesel::table! {
    workspaces (id) {
        id -> Uuid,
        #[max_length = 40]
        name -> Varchar,
        description -> Nullable<Text>,
        member_count -> Int4,
        image_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(project_members -> projects (project));
diesel::joinable!(project_members -> users (member));
diesel::joinable!(projects -> workspaces (workspace));
diesel::joinable!(workspace_members -> users (member));
diesel::joinable!(workspace_members -> workspaces (workspace));

diesel::allow_tables_to_appear_in_same_query!(
    project_members,
    projects,
    users,
    workspace_members,
    workspaces,
);
