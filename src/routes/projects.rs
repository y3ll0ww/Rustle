mod get;
mod post;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get::get_projects_of_current_user, // GET:     /projects
        post::create_new_project_by_form,  // POST:    /projects/new/<workspace>
        get::get_project_by_id,            // GET:     /projects/<id>
    ]
}
