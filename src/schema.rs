// @generated automatically by Diesel CLI.

diesel::table! {
    user (user_id) {
        user_id -> Text,
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
