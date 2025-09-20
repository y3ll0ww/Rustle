import { useEffect, useState } from "react";
import { Workspaces } from "../utils/ApiHandler";
import NoWorkspacesPage from "./NoWorkspacePage";
import LoadingPage from "./LoadingPage";
import WorkspaceGrid from "./components/WorkspaceGrid";

export default function DashboardPage() {
    const [workspaces, setWorkspaces] = useState([]);
    const [loading, setLoading] = useState(true);

    // Function for fetching workspaces of user
    const getWorkspaces = async () => {
      try {
        const data = await Workspaces.from_user();
        console.log(data);
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
        return <NoWorkspacesPage />;
    }

    // Return the content of the page
    return <WorkspaceGrid workspaces={workspaces} />;
}
