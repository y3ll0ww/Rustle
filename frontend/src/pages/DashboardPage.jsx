import { useNavigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext";
import { api } from "../utils/ApiHandler";

export default function DashboardPage() {
    const navigate = useNavigate();
    const { logout } = useAuth();

    const handleClick = async (e) => {
        e.preventDefault();
        await logout();
        navigate("/login");
    };

    return (
        <div>
            <div>Protected Area</div>
            <button onClick={(e) => handleClick(e)}>Logout</button>
        </div>
    )
}
