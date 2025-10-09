import "../../style/sidebar.css";
import { useState } from "react";
import { Link } from "react-router-dom";
import { BrainCircuit, Menu, LogOut, Home, Settings, Users } from "lucide-react";
import { Endpoint } from "../../utils/EndPoints";
import { useTheme } from "../../context/ThemeContext";
import LogoutModal from "./ModalLogout";

export default function Sidebar() {
  const { logo, logoIcon } = useTheme();
  const [logoutModalOpen, setLogoutModalOpen] = useState(false);
  const [sidebarOpen, setSidebarOpen] = useState(true);

  function Header() {
    return <div className="sidebar-header">
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
  }

  function Navigation() {
    const navLinks = [
      { to: Endpoint.home, label: "Dashboard", icon: Home },
      { to: Endpoint.workspaces, label: "Workspaces", icon: BrainCircuit },
      { to: "/users", label: "Users", icon: Users },
      { to: "/settings", label: "Settings", icon: Settings },
    ];

    return <nav className="sidebar-nav">
      {navLinks.map(({ to, label, icon: Icon }) => (
        <Link key={to} to={to} className="nav-link">
          <Icon size={20} />
          {sidebarOpen && <span>{label}</span>}
        </Link>
      ))}
    </nav>
  }

  function Footer() {
    return (
      <div className="sidebar-footer">
        <button
          className="logout-btn"
          onClick={() => setLogoutModalOpen(true)}
        >
          <LogOut size={18} />
          {sidebarOpen && <span>Logout</span>}
        </button>

        <LogoutModal
          isOpen={logoutModalOpen}
          onClose={() => setLogoutModalOpen(false)}
        />
      </div>
    )
  }

  return (
    <aside className={`sidebar ${sidebarOpen ? "open" : "collapsed"}`}>
      <Header />
      <Navigation />
      <Footer />
    </aside>
  );
}
