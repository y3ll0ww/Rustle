pub mod teams;
pub mod users;

pub const TEAMS: &str = "/teams/";
pub const USERS: &str = "/user/";

pub fn teams() -> Vec<rocket::Route> {
    routes![teams::new, teams::overview, teams::get_team, teams::delete,]
}

// Routes should be as follows:
// * /user               -> GET    (List users)
// * /user/<id>          -> GET    (Get specific user)
// * /user/<id>/update   -> PUT    (Update user info)
// * /user/<id>/delete   -> DELETE (Delete user)
// * /user/login         -> POST   (Login user)
// * /user/logout        -> POST   (Logout user)
// * /user/register      -> POST   (Register user)

pub fn users() -> Vec<rocket::Route> {
    routes![
        users::all,
        users::register,
        users::create,
        users::get,
        users::delete,
        users::login,
        users::logout,
    ]
}
