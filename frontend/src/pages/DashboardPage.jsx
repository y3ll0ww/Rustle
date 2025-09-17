import { useNavigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext";
import { Endpoint } from "../utils/EndPoints";

export default function DashboardPage() {
    const navigate = useNavigate();
    const { logout } = useAuth();

    const handleClick = async (e) => {
        e.preventDefault();
        await logout();
        navigate(Endpoint.home);
    };

    return (
        <div>
            <div>Protected Area</div>
            <button onClick={(e) => handleClick(e)}>Logout</button>
        </div>
    )
}
