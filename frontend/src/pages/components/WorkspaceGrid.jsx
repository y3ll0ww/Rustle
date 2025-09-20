import "../../style/cardgrid.css";
import WorkspaceCard from "./WorkspaceCard";

const MAX_WORKSPACES_IN_GRID = 3;

export default function WorkspaceGrid({ workspaces }) {
    return (
        <div>
            <h2 style={{ marginLeft: "1.5rem"}}>Workspaces</h2>
            <div className="card-grid">
                {workspaces.slice(0, MAX_WORKSPACES_IN_GRID).map((workspace) => (
                    <WorkspaceCard workspace={workspace} key={workspace.id} />
                ))}
            </div>
        </div>
    );
}