import WorkspaceListPage from "./WorkspacesListPage";

export default function DashboardPage() {
    const MAX_WORKSPACES_IN_GRID = 3;

    // Return the content of the page
    return <div className="flex w-screen">
        <h1>Dashboard</h1>
        {/*<WorkspaceGrid workspaces={workspaces} />*/}
        <WorkspaceListPage cap={MAX_WORKSPACES_IN_GRID}/>
    </div>;
}
