use crate::{routes::{PROJECTS, WORKSPACES}, tests::workspaces::TARGETED_WORKSPACE};

#[cfg(test)]
mod adding_and_updating;
#[cfg(test)]
mod deleting_projects;
#[cfg(test)]
mod getting_projects;

const TARGETED_PROJECT: &str = "3d035df3-2ea9-4731-bbd7-3ca5f270b9cf";

fn route_get_projects_from_workspace() -> String {
    format!("{WORKSPACES}{TARGETED_WORKSPACE}/projects")
}

fn route_projects_get() -> String {
    format!("{PROJECTS}{TARGETED_PROJECT}")
}

fn route_projects_create() -> String {
    format!("{PROJECTS}/new/{TARGETED_WORKSPACE}")
}

fn route_projects_delete() -> String {
    format!("{PROJECTS}{TARGETED_PROJECT}/delete")
}
