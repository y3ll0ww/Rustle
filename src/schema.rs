// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Nullable<Binary>,
        username -> Text,
        display_name -> Nullable<Text>,
        email -> Text,
        password_hash -> Text,
        bio -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
