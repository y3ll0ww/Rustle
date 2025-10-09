import "../style/login.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../context/AuthContext";
import { User } from "../utils/ApiHandler";
import { Endpoint } from "../utils/EndPoints";
import { LogInIcon, XIcon } from "lucide-react";
import { useTheme } from "../context/ThemeContext";

export default function LoginPage() {
  const { logo } = useTheme();
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const navigate = useNavigate();
  const { check_session } = useAuth();

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError(""); // reset any previous error

    try {
      await User.login({ username, password });
      await check_session();
      navigate(Endpoint.home);
    } catch (err) {
      console.error("Login error:", err);
      // Set a user-friendly error message
      setError("Invalid username or password. Please try again.");
    }
  };

  return (
    <div className="login-page">

      <img
        src={logo()}
        alt="Logo"
        className="login-logo"
      />

      <div className="content-container">
        <h1>Login</h1>

        {error && <div className="error-box">
          <span>{error}</span>
          <button
            type="button"
            className="error-close"
            onClick={() => setError("")}
            aria-label="Close error"
          >
            <XIcon size={16} />
          </button>
        </div>}

        <form className="login-form" onSubmit={handleSubmit}>
          <input
            type="text"
            placeholder="Username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
          />
          <input
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />

          <button className="btn-primary" type="submit">
            <LogInIcon size={18} />
            <span>Log In</span>
          </button>
        </form>
      </div>
    </div>
  );
}
