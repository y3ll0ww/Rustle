import "../assets/dashboard.css";
import React, { useEffect, useState } from "react";
import { Outlet, Link } from "react-router-dom";
import { Menu, ChevronDown, LogOut, Home, Settings, Users, Search } from "lucide-react";
import { useAuth } from "../context/AuthContext";

export default function DashboardLayout() {
  const [theme, setTheme] = useState("dark");

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
  }, [theme]);

  const { user, logout } = useAuth();
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [profileOpen, setProfileOpen] = useState(false);

  const navLinks = [
    { to: "/dashboard", label: "Dashboard", icon: Home },
    { to: "/users", label: "Users", icon: Users },
    { to: "/settings", label: "Settings", icon: Settings },
  ];

  return (
    <div className="dashboard-layout">
      <button
        onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
        className="absolute top-4 right-4 px-3 py-1 rounded bg-indigo-600 text-white"
      >
        Toggle Theme
      </button>

      {/* Sidebar */}
      <aside className={`sidebar ${sidebarOpen ? "open" : "collapsed"}`}>
        <div className="sidebar-header">
          {sidebarOpen && <span className="logo">Rustle</span>}
          <button className="menu-btn" onClick={() => setSidebarOpen(!sidebarOpen)}>
            <Menu size={22} />
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

      {/* Main content */}
      <div className="main-content">
        {/* Topbar */}
        <header className="topbar">
          <div className="search-bar">
            <Search size={18} />
            <input type="text" placeholder="Search..." />
          </div>

          <div className="profile-dropdown">
            <button onClick={() => setProfileOpen(!profileOpen)} className="profile-btn">
              <img
                src={`https://ui-avatars.com/api/?name=${user?.username || "U"}`}
                alt="avatar"
              />
              {sidebarOpen && <span>{user?.username || "Guest"}</span>}
              <ChevronDown size={16} />
            </button>

            {profileOpen && (
              <div className="dropdown-menu">
                <button onClick={logout}>Logout</button>
              </div>
            )}
          </div>
        </header>

        {/* Page content */}
        <main className="page-content">
          <div className="content-wrapper">
            <Outlet />
          </div>
        </main>
      </div>
    </div>
  );
}
