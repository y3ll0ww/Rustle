import BgImg from "../assets/20945431.png";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext";
import { Endpoint } from "../utils/EndPoints";
import { Plus } from "lucide-react";

export default function NoWorkspacesPage() {
    const navigate = useNavigate();
    const { user, logout } = useAuth();

    const handleClick = async (e) => {
        e.preventDefault();
        await logout();
        navigate(Endpoint.home);
    };

    return (
        <div className="flex flex-col items-center h-screen">
            <img
              src={BgImg}
              width="600px"
              alt="illustration"
              className="grayscale"
            />
            <h1>No workspaces found</h1>

            {user?.role >= 100 ? (
                <>
                    <p style={{ marginTop: "2em", marginBottom: "2em" }}>
                        You don't have any workspaces right now. Create one to get started.
                    </p>
                    <button
                        onClick={handleClick}
                        className="btn-primary flex items-center gap-2"
                    >
                        <Plus size={18} />
                        <span>Create Workspace</span>
                    </button>
                </>
            ) : (
                <p style={{ marginTop: "2em", marginBottom: "2em" }}>
                    You don't have access to create workspaces. Please wait until you're
                    invited to join one.
                </p>
            )}
        </div>
    )
}
