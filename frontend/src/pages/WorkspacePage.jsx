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
  const [title, setTitle] = useState(null);
  const [description, setDescription] = useState(null);

  // Function for fetching workspace information
  const getWorkspaceById = async () => {
    try {
      const data = await Workspaces.by_id(id);
      setWorkspace(data.workspace || null);
      setTitle(data.workspace?.name || null);
      setDescription(data.workspace?.description || null);
    } catch {
      setWorkspace(null);
      setTitle(null);
      setDescription(null);
    } finally {
      setLoading(false);
    }
  };

  const saveEditing = async () => {
    setLoading(true);
    try {
      const data = await Workspaces.update_information({
        id: id,
        updated_info: {
          name: title,
          description: description,
        }
      });
      console.log("DATA:");
      console.log(data);
      setIsEditing(false);
    } catch {
      setTitle(workspace?.name);
      setDescription(workspace?.description);
      setIsEditing(false);
    } finally {
      setLoading(false);
    }
  }

  function cancelEditing() {
    setTitle(workspace.name);
    setDescription(workspace.description);
    setIsEditing(false);
  }

  // Get user's workspaces via API
  useEffect(() => {
    // Initial fetch
    getWorkspaceById();

    // Poll every 30s
    const interval = setInterval(() => {
      if (isEditing) {
        getWorkspaceById();
      }
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
      {/* Full width header */}
      <header className="workspace-header">
        <Title
          name={title}
          onChange={setTitle}
          created_at={workspace.created_at}
          updated_at={workspace.updated_at}
          isEditing={isEditing}
        />
        {isEditing ? (
          <div className="workspace-header">
            <button className="btn-primary" onClick={saveEditing}>
              <Save size={16} />
              <span> Save</span>
            </button>
            <button className="btn-primary" onClick={cancelEditing}>
              <Ban size={16} />
              <span> Cancel</span>
            </button>
          </div>
        ) : (
          <button className="btn-primary" onClick={() => setIsEditing(true)}>
            <Pencil size={16} />
            <span> Edit</span>
          </button>
        )}
      </header>

      {/* Two column layout */}
      <div className="workspace-body">
        <div className="workspace-main">
          {isEditing ? (
            <MarkdownEditor value={description} onChange={setDescription} />
          ) : (
            <div className="content-container" style={{ height: "calc(100vh - 220px)" }}>
              <div className="markdown">
                <ReactMarkdown remarkPlugins={[remarkGfm]}>{description}</ReactMarkdown>
              </div>
            </div>
          )}
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
      </div>
    </div>
  );
}
