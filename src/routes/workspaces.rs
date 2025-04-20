mod delete;
mod get;
mod post;
mod put;

// * /workspaces               -> GET
// * /workspaces/new           -> POST
// * /workspaces/<id>          -> GET
// * /workspaces/update/<id>   -> PUT
// * /workspaces/delete/<id>   -> DELETE
pub fn routes() -> Vec<rocket::Route> {
    routes![
        get::get_list_of_workspaces_by_user_id,
        post::create_new_workspace_by_form,
        get::get_workspace_by_id,
        put::update_workspace,
        delete::delete_workspace_by_id,
    ]
}
