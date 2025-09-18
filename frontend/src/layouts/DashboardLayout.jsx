import "../assets/dashboard.css";
import LogoDark from "../assets/logo-dark.png";
import LogoLight from "../assets/logo-light.png";
import LogoDarkIcon from "../assets/logo-dark-icon.png";
import LogoLightIcon from "../assets/logo-light-icon.png";
import React, { useEffect, useState, useRef } from "react";
import { Outlet, Link } from "react-router-dom";
import { Menu, ChevronDown, LogOut, Home, Settings, Users, Search, SunMoon } from "lucide-react";
import { useAuth } from "../context/AuthContext";
import { Endpoint } from "../utils/EndPoints";

export default function DashboardLayout() {
  const { user, logout } = useAuth();
  const [theme, setTheme] = useState("dark");
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [profileOpen, setProfileOpen] = useState(false);
  const dropdownRef = useRef(null);

  // Close dropdown on outside click
  useEffect(() => {
    function handleClickOutside(e) {
      if (dropdownRef.current && !dropdownRef.current.contains(e.target)) {
        setProfileOpen(false);
      }
    }
    if (profileOpen) {
      document.addEventListener("mousedown", handleClickOutside);
    } else {
      document.removeEventListener("mousedown", handleClickOutside);
    }
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, [profileOpen]);

  function handleEscape(e) {
    if (e.key === "Escape") setProfileOpen(false);
  }

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", theme);
  }, [theme]);

  const navLinks = [
    { to: "/dashboard", label: "Dashboard", icon: Home },
    { to: "/users", label: "Users", icon: Users },
    { to: "/settings", label: "Settings", icon: Settings },
  ];

  return (
    <div className="dashboard-layout">

      {/* Sidebar */}
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
                src={theme === "dark" ? LogoLight : LogoDark}
                width="100px"
                alt="Logo"
              />
            </Link>
          }
          <button className="menu-btn" onClick={() => setSidebarOpen(!sidebarOpen)}>
            {sidebarOpen
              ? <Menu size={22} />
              : <img
                src={theme === "dark" ? LogoLightIcon : LogoDarkIcon}
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

      {/* Main content */}
      <div className="main-content">
        {/* Topbar */}
        <header className="topbar">
          <div className="search-bar">
            <Search size={18} />
            <input type="text" placeholder="Search..." />
          </div>

          <div className="profile-dropdown" ref={dropdownRef}>
            <button
              onClick={() => setProfileOpen(!profileOpen)}
              className="profile-btn"
            >
              <img
                src={`https://ui-avatars.com/api/?name=${user?.username || "U"}`}
                alt="avatar"
              />
              {sidebarOpen && <span>{user?.username || "Guest"}</span>}
              <ChevronDown size={16} />
            </button>

            {profileOpen && (
              <div className="dropdown-menu">
                <button onClick={() => setTheme(theme === "dark" ? "light" : "dark")}>
                  <SunMoon size={22} />
                  <span>Toggle Theme</span>
                </button>
                <button onClick={logout}>
                  <LogOut size={22} />
                  <span>Logout</span>
                </button>
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
