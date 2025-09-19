import { useEffect, useState } from "react";
import { Workspaces } from "../utils/ApiHandler";
import NoWorkspacesPage from "./NoWorkspacePage";
import LoadingPage from "./LoadingPage";

export default function DashboardPage() {
    const [workspaces, setWorkspaces] = useState([]);
    const [loading, setLoading] = useState(true);

    // Get user's workspaces via API
    useEffect(() => {
      const getWorkspaces = async () => {
        try {
          const data = await Workspaces.from_user();
          setWorkspaces(data || []);
        } catch {
          setWorkspaces([]);
        } finally {
          setLoading(false);
        }
      }

      getWorkspaces();
    }, []);

    // Return loading screen when the workspaces are being fetched
    if (loading) {
      return <div className="flex items-center justify-center h-full">
        <LoadingPage />
      </div>;
    }

    // Return the content of the page
    return (
      <>
        {workspaces.length === 0 ? (
          <NoWorkspacesPage />
        ) : (
          <div>{/* workspace UI */}</div>
        )}
      </>
    )
}
