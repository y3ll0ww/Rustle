// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        user_role -> Text,
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
