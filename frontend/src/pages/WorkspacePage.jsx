import "../style/markdown.css";
import "../style/page-workspace.css";
import { useEffect, useRef, useState } from "react";
import { useNavigate, useParams, useSearchParams } from "react-router-dom";
import { Workspaces } from "../utils/ApiHandler";
import NoWorkspacesPage from "./NoWorkspacePage";
import Title from "./components/Title";
import ProjectsPaginatedPage from "./ProjectsPaginated";
import UsersPaginatedList from "./components/UsersPaginatedList";
import { usePagination } from "../hooks/usePagination";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { EllipsisVertical, PackageSearchIcon, UserRoundSearchIcon } from "lucide-react";
import MarkdownEditor from "./components/MarkdownEditor";
import WorkspaceDropDown from "./components/WorkspaceDropdown";
import { useAlert } from "../context/AlertContext";
import DeleteWorkspaceModal from "./components/modal/DeleteWorkspaceModal";
import { Endpoint } from "../utils/EndPoints";
import Loading from "../components/Loading";
import WorkspaceUsersPage from "./components/WorkspaceUsers";
import { usePageTitle } from "../hooks/usePageTitle";

export default function WorkspacePage() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [searchParams, setSearchParams] = useSearchParams();
  const [workspace, setWorkspace] = useState(null);
  const [loading, setLoading] = useState(true);
  const { paginationState, updateFromResponse } = usePagination();
  const [isEditing, setIsEditing] = useState(false);
  const [title, setTitle] = useState(null);
  const [description, setDescription] = useState(null);
  const [dropdownOpen, setDropdownOpen] = useState(false);
  const [deleteModalOpen, setDeleteModalOpen] = useState(false);
  const dropdownRef = useRef(null);
  const { alertSuccess, alertInfo, alertError } = useAlert();

  usePageTitle(title);

  // Determine which tab to show from URL
  const tab = searchParams.get("tab");
  const showUsers = tab === "users";

  // Fetch workspace data
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

  const handleSave = async () => {
    setLoading(true);
    try {
      const data = await Workspaces.update_information({
        id: id,
        updated_info: { name: title, description: description },
      });
      setWorkspace(data);
      setIsEditing(false);
      alertSuccess(`${workspace.name} updated`, "Workspace info updated.");
    } catch {
      setTitle(workspace?.name);
      setDescription(workspace?.description);
      setIsEditing(false);
      alertError("Error updating workspace");
    } finally {
      setLoading(false);
    }
  };

  function handleCancel() {
    setTitle(workspace.name);
    setDescription(workspace.description);
    setIsEditing(false);
  }

  useEffect(() => {
    function handleClickOutside(e) {
      if (dropdownRef.current && !dropdownRef.current.contains(e.target)) {
        setDropdownOpen(false);
      }
    }
    function handleEscape(e) {
      if (e.key === "Escape") setDropdownOpen(false);
    }
    if (dropdownOpen) {
      document.addEventListener("mousedown", handleClickOutside);
      document.addEventListener("keydown", handleEscape);
    } else {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleEscape);
    }
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleEscape);
    };
  }, [dropdownOpen]);

  // Handle deleting a workspace
  const handleDelete = async (e) => {
    e.preventDefault();
    try {
      await Workspaces.delete(id);
      navigate(Endpoint.workspaces);
      alertInfo("Workspace deleted", `Deleted workspace '${id}'`);
    } catch {
      alertError("Error deleting workspace", "Try contacting your administrator.");
    }
  };

  // Show Users (updates URL)
  const handleShowUsers = () => setSearchParams({ tab: "users" });
  const handleBackFromUsers = () => setSearchParams({}); // remove tab

  useEffect(() => {
    getWorkspaceById();
    const interval = setInterval(() => {
      if (isEditing) getWorkspaceById();
    }, 30000);
    return () => clearInterval(interval);
  }, []);

  if (loading) return <Loading size="large" />;
  if (!workspace) return <NoWorkspacesPage />;

  return (
    <div className="workspace-page">
      {/* Full width header */}
      <header className="workspace-header">
        <Title
          name={title}
          onChange={setTitle}
          image_url={workspace.image_url}
          created_at={workspace.created_at}
          updated_at={workspace.updated_at}
          isEditing={isEditing}
        />
      </header>

      {/* Dropdown menu */}
      {!isEditing && (
        <div
          ref={dropdownRef}
          style={{ cursor: "pointer" }}
          className="edit-ws-button"
          onClick={() => setDropdownOpen(!dropdownOpen)}
        >
          <EllipsisVertical size={22} />
          {dropdownOpen && (
            <WorkspaceDropDown
              handleEdit={() => setIsEditing(true)}
              handleDelete={() => setDeleteModalOpen(true)}
              handleShowUsers={handleShowUsers}
            />
          )}
        </div>
      )}

      {/* Two column layout */}
      <div className="workspace-body">
        <div className="workspace-main">
          {isEditing ? (
            <MarkdownEditor
              value={description}
              onChange={setDescription}
              onSave={handleSave}
              onCancel={handleCancel}
            />
          ) : showUsers ? (
            <div style={{ height: "100%" }}>
              <WorkspaceUsersPage handleBack={handleBackFromUsers} />
            </div>
          ) : (
            <div className="markdown-final markdown">
              <ReactMarkdown remarkPlugins={[remarkGfm]}>
                {description}
              </ReactMarkdown>
            </div>
          )}
        </div>

        {/* Right sidebar */}
        {!isEditing && !showUsers && (
          <div className="right-sidebar">
            <div className="inline-pill">
              <h2>Projects</h2>
              <button
                className="btn-secondary pill"
              >
                <PackageSearchIcon size={20} />
              </button>
            </div>
            <ProjectsPaginatedPage workspace_id={workspace.id} />

            <div className="inline-pill">
              <h2>Members</h2>
              <button
                className="btn-secondary pill"
                onClick={handleShowUsers}
              >
                  <UserRoundSearchIcon size={20} />
              </button>
            </div>

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

      {/* Delete Modal */}
      <DeleteWorkspaceModal
        isOpen={deleteModalOpen}
        onClose={() => setDeleteModalOpen(false)}
        onSubmit={handleDelete}
      />
    </div>
  );
}
