import "../../assets/dashboard.css";
import { useState } from "react";
import { Link } from "react-router-dom";
import { Menu, LogOut, Home, Settings, Users } from "lucide-react";
import { useAuth } from "../../context/AuthContext";
import { Endpoint } from "../../utils/EndPoints";
import { useTheme } from "../../context/ThemeContext";

export default function Sidebar() {
  const { logout } = useAuth();
  const { logo, logoIcon } = useTheme();
  const [sidebarOpen, setSidebarOpen] = useState(true);

  const navLinks = [
    { to: "/dashboard", label: "Dashboard", icon: Home },
    { to: "/users", label: "Users", icon: Users },
    { to: "/settings", label: "Settings", icon: Settings },
  ];

  return (
    <aside className={`sidebar ${sidebarOpen ? "open" : "collapsed"}`}>
      <div className="sidebar-header">
        {sidebarOpen &&
          <Link
            to={Endpoint.home}
            style={{
              all: "unset",
              cursor: "pointer",
              display: "inline",
              marginTop: "4px",
            }}
          >
            <img
              src={logo()}
              width="100px"
              alt="Logo"
            />
          </Link>
        }
        <button className="menu-btn" onClick={() => setSidebarOpen(!sidebarOpen)}>
          {sidebarOpen
            ? <Menu size={22} />
            : <img
              src={logoIcon()}
              width="22px"
              alt="Expand Sidebar"
            />
          }
        </button>
      </div>

      <nav className="sidebar-nav">
        {navLinks.map(({ to, label, icon: Icon }) => (
          <Link key={to} to={to} className="nav-link">
            <Icon size={20} />
            {sidebarOpen && <span>{label}</span>}
          </Link>
        ))}
      </nav>

      <div className="sidebar-footer">
        <button onClick={logout} className="logout-btn">
          <LogOut size={18} />
          {sidebarOpen && <span>Logout</span>}
        </button>
      </div>
    </aside>
  );
}
