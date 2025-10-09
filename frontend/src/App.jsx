import { Link, useNavigate } from "react-router-dom";
import { Endpoint } from "./utils/EndPoints";
import { LogInIcon } from "lucide-react";
import { useTheme } from "./context/ThemeContext";

function App() {
  const { logo } = useTheme();
  const navigate = useNavigate();

  return (
    <div style={{
      height: "100vh",
      width: "100vw",
      display: "flex",
      alignItems: "center",
      justifyContent: "center",
      textAlign: "center",
      animation: "fadeIn 0.6s ease",
    }}>
      <div style={{
        padding: "3rem 4rem",
        maxWidth: "600px",
        width: "90%",
      }}>
        <img
          src={logo()}
          alt="Logo"
          style={{ width: "300px" }}
        />
        <h1>Welcome to Rustle</h1>
        <p style={{ margin: "1rem 0rem 2rem 0rem" }}>This is your main landing page.</p>
        <button
          className="btn-primary"
          onClick={() => navigate(Endpoint.login)}
          style={{ padding: "0.5rem 2rem 0.5rem 2rem" }}
        >
          <LogInIcon />
          <span>Go to Login</span>
        </button>
      </div>
    </div>
  );
}

export default App;
