export const WorkspaceRoles = {
  MEMBER: 0,
  STAKEHOLDER: 1,
  CONTRIBUTOR: 2,
  MANAGER: 5,
  OWNER: 10,
};

// Function to resolve role name
export const workspaceMemberType = (role) => {
    /// Maximum privileges; only one able to delete a workspace
  if (role >= WorkspaceRoles.OWNER) return "Owner";
  /// High-level privileges; can manage members, settings, and permissions but cannot delete the workspace
  if (role >= WorkspaceRoles.MANAGER) return "Manager";
  /// Can contribute work but has limited administrative privileges
  if (role >= WorkspaceRoles.CONTRIBUTOR) return "Contributor";
  /// Can review and approve work but cannot make direct contributions
  if (role >= WorkspaceRoles.STAKEHOLDER) return "Stakeholder";
  /// Limited access, can only view but not interact with content
  return "Member";
};

// Function to resolve role color
export const workspaceRoleColor = (role) => {
  if (role >= WorkspaceRoles.OWNER) return "red";
  if (role >= WorkspaceRoles.MANAGER) return "orange";
  return "inherit";
};
