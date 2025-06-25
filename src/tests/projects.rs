use crate::{
    routes::{PROJECTS, WORKSPACES},
    tests::{root_route, workspaces::TARGETED_WORKSPACE},
};

#[cfg(test)]
mod adding_and_updating;
#[cfg(test)]
mod deleting_projects;
#[cfg(test)]
mod getting_projects;
#[cfg(test)]
mod member_management;

const TARGETED_PROJECT: &str = "3465a06a-994f-4467-a6c4-3e949cf5e21b";

fn route_get_projects_by_user() -> String {
    root_route(PROJECTS)
}

fn route_get_projects_from_workspace() -> String {
    format!("{WORKSPACES}{TARGETED_WORKSPACE}/projects")
}

fn route_get_projects_from_workspace_paginated() -> String {
    format!("{WORKSPACES}{TARGETED_WORKSPACE}/projects/browse")
}

fn route_projects_get() -> String {
    format!("{PROJECTS}{TARGETED_PROJECT}")
}

fn route_projects_create() -> String {
    format!("{PROJECTS}{TARGETED_WORKSPACE}/new")
}

fn route_projects_delete() -> String {
    format!("{PROJECTS}{TARGETED_PROJECT}/delete")
}

fn route_projects_add_member() -> String {
    format!("{PROJECTS}{TARGETED_PROJECT}/add-members")
}

fn route_projects_remove_member(id: &str) -> String {
    format!("{PROJECTS}{TARGETED_PROJECT}/remove-member/{id}")
}
