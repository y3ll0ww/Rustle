mod database;
mod delete;
mod get;
mod post;

// * /user               -> GET
// * /user/<id>          -> GET
// * /user/<id>/update   -> PUT
// * /user/<id>/delete   -> DELETE
// * /user/login         -> POST
// * /user/logout        -> POST
// * /user/register      -> POST
// * /user/invite        -> POST
pub fn routes() -> Vec<rocket::Route> {
    routes![
        get::list_all_users,
        get::get_user_by_username,
        delete::delete_user_by_id,
        post::login_by_form,
        post::logout,
        post::create_new_user_by_form,
        post::invite_new_users_by_form,
        post::inject_user,
    ]
}
