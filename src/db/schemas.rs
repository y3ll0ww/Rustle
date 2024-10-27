use super::table;

table! {
    users (user_id) {
        user_id -> VarChar,
        user_role -> VarChar,
        username -> Varchar,
        display_name -> Nullable<Varchar>,
        email -> Varchar,
        password_hash -> Varchar,
        bio -> Nullable<Text>,
        avatar_url -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
