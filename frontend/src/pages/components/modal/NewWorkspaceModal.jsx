import { BanIcon, SaveIcon } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { useAlert } from "../../../context/AlertContext";
import { Workspaces } from "../../../utils/ApiHandler";
import { useNavigate } from "react-router-dom";
import { Endpoint } from "../../../utils/EndPoints";
import Modal from "./Modal";

export default function NewWorkspaceModal({ isOpen, onClose, onSubmit }) {
    const navigate = useNavigate();
    const [workspaceName, setWorkspaceName] = useState("");
    const { alertSuccess, alertError } = useAlert();
    const inputRef = useRef();

    useEffect(() => {
        if (isOpen && inputRef.current) {
            inputRef.current.focus();
        }
    }, [isOpen]);

    const handleSubmit = async (e) => {
        e.preventDefault();

        try {
            const data = await Workspaces.create({ name: workspaceName, description: defaultDescription() });
            navigate(`${Endpoint.workspace}/${data.workspace.id}`)
            alertSuccess(`Created '${workspaceName}'`, "Workspace created successfully. You're added as the owner.");
        } catch (err) {
            alertError("Failed to create workspace.");
        }

        onClose();
        setWorkspaceName("");
        onSubmit();
    };

    return <Modal
        title="Create new Workspace"
        content={<form onSubmit={handleSubmit}>
            <input
                ref={inputRef}
                type="text"
                placeholder="Enter a name for the new workspace..."
                className="modal-input"
                value={workspaceName}
                onChange={(e) => setWorkspaceName(e.target.value)}
                onKeyDown={(e) => {
                    if (e.key === "Enter") {
                        e.preventDefault();
                        handleSubmit(e);
                    }
                }}
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
        </form>}
        isOpen={isOpen}
        onClose={onClose}
    />;
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
