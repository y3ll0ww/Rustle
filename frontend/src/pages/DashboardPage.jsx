import { useNavigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext";

export default function DashboardPage() {
    const navigate = useNavigate();
    const { logout } = useAuth();

    const handleClick = async (e) => {
        e.preventDefault();

        try {
            const res = await fetch(`${import.meta.env.VITE_API_URL}/user/logout`, {
                method: "POST",
                headers: { "Content-Type": "application/x-www-form-urlencoded" },
                credentials: "include",
            });

            if (res.ok) {
                console.log(res);
            }

            if (!res.ok) throw new Error("Logout failed");

            await logout();
            navigate("/login");
        } catch (err) {
            console.error("Logout error:", err);
            alert("Logout failed.");
        }
    };

    return (
        <div>
            <div>Protected Area</div>
            <button onClick={(e) => handleClick(e)}>Logout</button>
        </div>
    )
}
