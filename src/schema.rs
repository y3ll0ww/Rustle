// @generated automatically by Diesel CLI.

diesel::table! {
    team_members (team_id, user_id) {
        team_id -> Uuid,
        user_id -> Uuid,
        team_role -> Int2,
    }
}

diesel::table! {
    team_updates (team_id) {
        team_id -> Uuid,
        last_updated -> Timestamp,
    }
}

diesel::table! {
    teams (id) {
        id -> Uuid,
        owner_id -> Uuid,
        #[max_length = 40]
        team_name -> Varchar,
        team_description -> Nullable<Text>,
        image_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        role -> Int2,
        status -> Int2,
        #[max_length = 40]
        username -> Varchar,
        #[max_length = 40]
        display_name -> Nullable<Varchar>,
        #[max_length = 100]
        email -> Varchar,
        password -> Text,
        bio -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(team_members -> teams (team_id));
diesel::joinable!(team_members -> users (user_id));
diesel::joinable!(team_updates -> teams (team_id));
diesel::joinable!(teams -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    team_members,
    team_updates,
    teams,
    users,
);
