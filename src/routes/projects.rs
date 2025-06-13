mod get;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get::get_projects_of_current_user, // GET:     /projects
    ]
}