import "../../style/modal.css";
import { useState } from "react";

export default function CreateWorkspaceModal({ isOpen, onClose, onCreate }) {
  const [workspaceName, setWorkspaceName] = useState("");

  if (!isOpen) return null;

  const handleSubmit = (e) => {
    e.preventDefault();
    if (!workspaceName.trim()) return;
    onCreate(workspaceName.trim());
    setWorkspaceName("");
    onClose();
  };

  // Prevent closing when clicking inside the modal
  const handleContentClick = (e) => {
    e.stopPropagation();
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div
        className="content-container"
        onClick={handleContentClick}
        style={{ maxWidth: "50%", padding: "2rem" }}
      >
        <h2 style={{ marginBottom: "2rem" }}>Create New Workspace</h2>
        <form onSubmit={handleSubmit}>
          <input
            type="text"
            placeholder="Workspace Name"
            value={workspaceName}
            onChange={(e) => setWorkspaceName(e.target.value)}
            style={inputStyle}
          />
          <div style={{ display: "flex", justifyContent: "flex-end", marginTop: "1rem", gap: "0.5rem" }}>
            <button className="btn-secondary" onClick={onClose}>
              Cancel
            </button>
            <button className="btn-primary">
              Create
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

const inputStyle = {
  width: "100%",
  padding: "0.5rem",
  fontSize: "1rem",
  borderRadius: "6px",
  border: "1px solid #ccc",
};

const buttonStylePrimary = {
  backgroundColor: "#007bff",
  color: "#fff",
  border: "none",
  borderRadius: "6px",
  padding: "0.5rem 1rem",
  cursor: "pointer",
};

const buttonStyleSecondary = {
  backgroundColor: "#eee",
  color: "#333",
  border: "none",
  borderRadius: "6px",
  padding: "0.5rem 1rem",
  cursor: "pointer",
};
