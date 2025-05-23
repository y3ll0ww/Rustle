mod delete;
mod get;
mod post;
mod put;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        get::get_list_of_workspaces_by_user_id, // GET:     /workspaces
        post::create_new_workspace_by_form,     // POST:    /workspaces/new
        get::get_workspace_by_id,               // GET:     /workspaces/<id>
        put::update_workspace,                  // PUT:     /workspaces/<id>/update
        post::add_members_to_workspace,         // POST:    /workspaces/<id>/add-members
        delete::delete_workspace_by_id,         // DELETE:  /workspaces/<id>/delete
        post::invite_new_users_to_workspace,    // POST:    /workspaces/<id>/invite
        post::reinvite_user_by_id,              // POST:    /workspaces/<id>/re-invite/<member>")]
        delete::remove_member_from_workspace,   // DELETE:  /workspaces/<id>/remove-member/<member>
    ]
}
