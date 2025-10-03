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
        <div
            style={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                height: "100vh",
                textAlign: "center",
                padding: "1rem",
            }}
        >
            <img
                src={BgImg}
                alt="illustration"
                style={{
                    width: "800px",
                    maxWidth: "70%",
                    filter: "grayscale(100%)",
                    marginBottom: "1rem",
                }}
            />
            <h1>No workspaces found</h1>

            {user?.role >= 100 ? (
                <>
                    <p style={{ margin: "2em 0" }}>
                        You don't have any workspaces right now. Create one to get started.
                    </p>
                    <button
                        onClick={handleClick}
                        className="btn-primary"
                    >
                        <Plus size={18} />
                        <span>Create Workspace</span>
                    </button>
                </>
            ) : (
                <p style={{ margin: "2em 0" }}>
                    You don't have access to create workspaces. Please wait until you're
                    invited to join one.
                </p>
            )}
        </div>

    )
}
