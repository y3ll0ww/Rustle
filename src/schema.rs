// @generated automatically by Diesel CLI.

diesel::table! {
    team_members (team_id, user_id) {
        team_id -> Text,
        user_id -> Text,
        team_privilege -> Integer,
    }
}

diesel::table! {
    team_updates (team_id) {
        team_id -> Text,
        last_updated -> Timestamp,
    }
}

diesel::table! {
    teams (id) {
        id -> Text,
        owner_id -> Text,
        team_name -> Text,
        team_description -> Nullable<Text>,
        image_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        privilege -> Integer,
        username -> Text,
        display_name -> Nullable<Text>,
        email -> Text,
        password_hash -> Text,
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

diesel::allow_tables_to_appear_in_same_query!(team_members, team_updates, teams, users,);
