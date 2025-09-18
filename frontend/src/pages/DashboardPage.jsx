import BgImg from "../assets/20945431.png";
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
        <div className="flex flex-col items-center h-screen">
            <img src={BgImg} width="500px" alt="illustration" className="grayscale" />
            <div className="mt-4 text-lg font-semibold">Protected Area</div>
            <button
                onClick={(e) => handleClick(e)}
                className="mt-4 px-4 py-2 bg-indigo-600 text-white rounded hover:bg-indigo-700"
            >
                Logout
            </button>
        </div>
    )
}
