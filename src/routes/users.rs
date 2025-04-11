mod database;
mod delete;
mod get;
mod post;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get::list_all_users,            // GET:     /user
        get::get_user_by_username,      // GET:     /user/<username>
        delete::delete_user_by_id,      // DELETE:  /user/<id>/delete
        post::login_by_form,            // POST:    /user/login
        post::logout,                   // POST:    /user/logout
        post::create_new_user_by_form,  // POST:    /user/register
        post::invite_new_users_by_form, // POST:    /user/invite
        post::inject_user,              // POST:    /user/create
    ]
}
