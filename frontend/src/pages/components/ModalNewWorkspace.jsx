import "../../style/modal.css";
import { BanIcon, SaveIcon, XIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { useAlert } from "../../context/AlertContext";
import { Workspaces } from "../../utils/ApiHandler";

export default function CreateWorkspaceModal({ isOpen, onClose, onCreate }) {
    const [workspaceName, setWorkspaceName] = useState("");
    const { alertSuccess, alertError } = useAlert();

    useEffect(() => {
        const handleEscape = (e) => {
            if (e.key === "Escape") onClose();
        };
        if (!isOpen) return;

        document.addEventListener("keydown", handleEscape);
        return () => document.removeEventListener("keydown", handleEscape);
    }, [isOpen, onClose]);

    if (!isOpen) return null;

    const handleSubmit = async (e) => {
        e.preventDefault();

        try {
            await Workspaces.create({ name: workspaceName, description: defaultDescription() });
            alertSuccess("Workspace created successfully!");
        } catch (err) {
            alertError("Failed to create workspace.");
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

const defaultDescription = () => {
    return `
# 🎉 Welcome to your new workspace!

Great to have you here.  
This workspace is your team’s central hub where projects, members, and ideas come together.  
You can get started right away with the options below.

---

## 🚀 Next Steps

| Action              | Why it matters                                      |
|---------------------|------------------------------------------------------|
| ➕ Create a project | Start organizing tasks, milestones, and deliverables |
| 👥 Invite members   | Bring in your teammates and collaborate effectively |
| 📝 Update settings  | Personalize your workspace name, logo, and details  |

---

## 💡 Tips

- Use **projects** to group related tasks and features.  
- Invite colleagues early to avoid working in silos.  
- Keep an eye on the dashboard for recent activity.  

---

✨ _Your workspace is ready. Let’s build something amazing together!_
    `;
};
