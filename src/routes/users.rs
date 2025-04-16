mod delete;
mod get;
mod post;
mod put;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get::list_all_users,            // GET:     /user
        get::get_user_by_username,      // GET:     /user/<username>
        post::inject_user,              // POST:    /user/create
        delete::delete_user_by_id,      // DELETE:  /user/delete/<id>
        post::invite_new_users_by_form, // POST:    /user/invite
        get::get_invited_user,          // GET:     /user/invite/get/<token>
        put::set_password_after_invite, // PUT:     /user/invite/set/<token>
        post::login_by_form,            // POST:    /user/login
        post::logout,                   // POST:    /user/logout
        post::create_new_user_by_form,  // POST:    /user/register
    ]
}
