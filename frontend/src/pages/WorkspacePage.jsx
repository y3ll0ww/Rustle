import "../style/page-workspace.css"
import "../style/markdown.css";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Workspaces } from "../utils/ApiHandler";
import NoWorkspacesPage from "./NoWorkspacePage";
import LoadingPage from "./LoadingPage";
import Title from "./components/Title";
import ProjectsPaginatedPage from "./ProjectsPaginated";
import UsersPaginatedList from "./components/UsersPaginatedList";
import { usePagination } from "../hooks/usePagination";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Ban, Pencil, Save } from "lucide-react";
import MarkdownEditor from "./components/MarkdownEditor";

export default function WorkspacePage() {
  const { id } = useParams();
  const [workspace, setWorkspace] = useState(null);
  const [loading, setLoading] = useState(true);
  const { paginationState, updateFromResponse } = usePagination();
  const [isEditing, setIsEditing] = useState(false);
  const [description, setDescription] = useState(null);

  // Function for fetching workspace information
  const getWorkspaceById = async () => {
    try {
      const data = await Workspaces.by_id(id);
      setWorkspace(data.workspace || null);
      setDescription(data.workspace?.description || null);
    } catch {
      setWorkspace(null);
      setDescription(null);
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
  return (
    <div className="workspace-page">
      <div className="workspace-main">
        <header>
          <Title
            name={workspace.name}
            created_at={workspace.created_at}
            updated_at={workspace.updated_at}
          />
          {isEditing
            ? <div>
              <button
                className="btn-primary"
                onClick={() => setIsEditing(false)}
              >
                <Save size={16} />
                <span> Save</span>
              </button>
              <button
                className="btn-primary"
                onClick={() => setIsEditing(false)}
              >
                <Ban size={16} />
                <span> Cancel</span>
              </button>
            </div>
            : <button
              className="btn-primary"
              onClick={() => setIsEditing(true)}
            >
              <Pencil size={16} />
              <span> Edit</span>
            </button>
          }
        </header>

        {isEditing
          ?
          <MarkdownEditor value={workspace.description} />
          : <div className="content-container" style={{ height: "calc(100vh - 220px)" }}>
            <div className="markdown">
              <ReactMarkdown remarkPlugins={[remarkGfm]}>{workspace.description}</ReactMarkdown>
            </div>
          </div>
        }
      </div>

      {!isEditing && (
        <div className="right-sidebar">
          <h2>Projects</h2>
          <ProjectsPaginatedPage workspace_id={workspace.id} />

          <h2>Members</h2>
          <UsersPaginatedList
            paginationState={paginationState}
            updateFromResponse={updateFromResponse}
            workspace={workspace.id}
            exclude_self={false}
            status={2}
          />
        </div>
      )}

    </div >
  );
}
