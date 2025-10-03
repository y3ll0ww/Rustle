import { BanIcon, SaveIcon, XIcon } from "lucide-react";
import "../../style/modal.css";
import { useState } from "react";
import { Workspaces } from "../../utils/ApiHandler";

export default function CreateWorkspaceModal({ isOpen, onClose, onCreate }) {
    const [workspaceName, setWorkspaceName] = useState("");

    if (!isOpen) return null;

    const handleSubmit = async (e) => {
        e.preventDefault();

        try {
            await Workspaces.create({ name: workspaceName });
        } catch (err) {
            console.error("Error creating workspace:", err);
            alert("Error creating workspace.");
        }

        onClose();
        setWorkspaceName("");
        onCreate();
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
            >
                <button
                    onClick={onClose}
                    aria-label="Close modal"
                    className="modal-close"
                >
                    <XIcon />
                </button>

                <h2 style={{ marginBottom: "2rem" }}>Create new Workspace</h2>
                <form onSubmit={handleSubmit}>
                    <input
                        type="text"
                        placeholder="Enter a name for the new workspace..."
                        className="modal-input"
                        value={workspaceName}
                        onChange={(e) => setWorkspaceName(e.target.value)}
                    />
                    <div className="modal-buttons">
                        <button className="btn-secondary" onClick={onClose}>
                            <BanIcon />
                            Cancel
                        </button>
                        <button className="btn-primary">
                            <SaveIcon />
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
