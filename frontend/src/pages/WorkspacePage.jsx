import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Workspaces } from "../utils/ApiHandler";
import NoWorkspacesPage from "./NoWorkspacePage";
import LoadingPage from "./LoadingPage";
import Title from "./components/Title";
import MemberListItem from "./components/MemberListItem";

export default function WorkspacePage() {
  const { id } = useParams();
  const [workspace, setWorkspace] = useState(null);
  const [members, setMembers] = useState([]);
  const [loading, setLoading] = useState(true);

  function Members() {
    return <div>
      <h2>Members</h2>
      <ul className="member-list">
        {members.map((member) => {
          const key = member.user.id;
          return <MemberListItem key={key} member={member} />;
        })}
      </ul>
    </div>
  }

  // Function for fetching workspace information
  const getWorkspaceById = async () => {
    try {
      const data = await Workspaces.by_id(id);
      console.log(data);
      setWorkspace(data.workspace || null);
      setMembers(data.members || []);
    } catch {
      setWorkspace(null);
      setMembers([]);
    } finally {
      setLoading(false);
    }
  };

  // Get user's workspaces via API
  useEffect(() => {
    // Initial fetch
    getWorkspaceById();

    // Poll every 30s
    const interval = setInterval(() => {
      getWorkspaceById();
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
  if (!workspace) {
    return <NoWorkspacesPage />;
  }

  // Return the content of the page
  return <div className="flex w-screen">
    <Title
      name={workspace.name}
      created_at={workspace.created_at}
      updated_at={workspace.updated_at}
    />
    <p>{workspace.description}</p>

    <Members />

  </div>;
}
