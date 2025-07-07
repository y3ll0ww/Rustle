mod delete;
mod get;
mod post;
mod put;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get::get_paginated_users,       // GET:     /user?<status>&<role>
        get::get_user_by_username,      // GET:     /user/<username>
        post::inject_user,              // POST:    /user/create
        put::remove_user,               // PUT:     /user/remove/<id>
        put::suspend_user,              // PUT:     /user/suspend/<id>
        put::update_user,               // PUT:     /user/update/<id>
        put::update_role,               // PUT:     /user/update/<id>/<role>
        delete::delete_user_by_id,      // DELETE:  /user/delete/<id>
        get::get_invited_user,          // GET:     /user/invite/get/<token>
        put::set_password_after_invite, // PUT:     /user/invite/set/<token>
        post::login_by_form,            // POST:    /user/login
        post::logout,                   // POST:    /user/logout
    ]
}
