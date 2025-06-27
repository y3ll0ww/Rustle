use crate::{routes::WORKSPACES, tests::root_route};

#[cfg(test)]
mod adding_and_updating;
#[cfg(test)]
mod getting_workspaces;
#[cfg(test)]
mod member_management;

pub const TARGETED_WORKSPACE: &str = "7fa5257b-e02b-4f6f-be9f-8f579fb64147";

fn route_workspaces_all() -> String {
    root_route(WORKSPACES)
}

fn route_workspaces_by_id() -> String {
    format!("{WORKSPACES}{TARGETED_WORKSPACE}")
}

pub fn route_workspaces_invite_to_workspace() -> String {
    format!("{WORKSPACES}{TARGETED_WORKSPACE}/invite")
}

fn route_workspaces_new() -> String {
    format!("{WORKSPACES}new")
}

fn route_workspaces_update() -> String {
    format!("{WORKSPACES}{TARGETED_WORKSPACE}/update")
}

fn route_workspaces_add_member() -> String {
    format!("{WORKSPACES}{TARGETED_WORKSPACE}/add-members")
}

fn route_workspaces_remove_member(user_id: &str) -> String {
    format!("{WORKSPACES}{TARGETED_WORKSPACE}/remove-member/{user_id}")
}
