import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Workspaces } from "../utils/ApiHandler";
import NoWorkspacesPage from "./NoWorkspacePage";
import LoadingPage from "./LoadingPage";
import ItemCard from "./components/ItemCard";
import { Endpoint } from "../utils/EndPoints";

export default function WorkspaceListPage({ cap, isSection }) {
    const navigate = useNavigate();
    const [workspaces, setWorkspaces] = useState([]);
    const [loading, setLoading] = useState(true);

    // Function for fetching workspaces of user
    const getWorkspaces = async () => {
        try {
            const data = await Workspaces.from_user();
            setWorkspaces(data || []);
        } catch {
            setWorkspaces([]);
        } finally {
            setLoading(false);
        }
    };

    // Get user's workspaces via API
    useEffect(() => {
        // Initial fetch
        getWorkspaces();

        // Poll every 30s
        const interval = setInterval(() => {
            getWorkspaces();
        }, 30000);

        // Cleanup interval on unmount
        return () => clearInterval(interval);
    }, []);

    // Return loading screen when the workspaces are being fetched
    if (loading) {
        return <div className="flex items-center justify-center h-full">
            <LoadingPage />
        </div>;
    }

    // Return no content page if user is not part of any workspace
    if (workspaces.length === 0) {
        return <NoWorkspacesPage getWorkspaces={getWorkspaces}/>;
    }

    const handleOpenWorkspace = async (id) => {
        navigate(`${Endpoint.workspace}/${id}`)
    }

    // Return the content of the page
    const max = cap === undefined ? workspaces.length : cap;
    return <div className="flex w-screen">
        <div>
            {isSection ? <h2>Workspaces</h2> : <h1>Workspaces</h1>}
            <div className="card-grid">
                {workspaces.slice(0, max).map((workspace) => (
                    <ItemCard
                        key={workspace.id}
                        type="Workspace"
                        item={workspace}
                        handleOpen={handleOpenWorkspace}
                    />
                ))}
            </div>
        </div>
    </div>;
}
