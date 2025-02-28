pub mod users;

// Routes should be as follows:
// * /user               -> GET    (List users)
// * /user/<id>          -> GET    (Get specific user)
// * /user/<id>/update   -> PUT    (Update user info)
// * /user/<id>/delete   -> DELETE (Delete user)
// * /user/login         -> POST   (Login user)
// * /user/logout        -> POST   (Logout user)
// * /user/register      -> POST   (Register user)

pub fn user() -> Vec<rocket::Route> {
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
