#[cfg(test)]
mod adding_and_updating;
#[cfg(test)]
mod getting_workspaces;
#[cfg(test)]
mod member_management;

const ROUTE_WORKSPACE: &str = "/workspaces/";
const ROUTE_WORKSPACES: &str = "/workspaces";
const ROUTE_WORKSPACE_NEW: &str = "/workspaces/new";

pub const TARGETED_WORKSPACE: &str = "7fa5257b-e02b-4f6f-be9f-8f579fb64147";
