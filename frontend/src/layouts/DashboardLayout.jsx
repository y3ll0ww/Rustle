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
import Sidebar from "./components/Sidebar";
import { useTheme } from "../context/ThemeContext";

export default function DashboardLayout() {
  const { user, logout } = useAuth();
  const { toggleTheme } = useTheme();
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

  const navLinks = [
    { to: "/dashboard", label: "Dashboard", icon: Home },
    { to: "/users", label: "Users", icon: Users },
    { to: "/settings", label: "Settings", icon: Settings },
  ];

  return (
    <div className="dashboard-layout">

      <Sidebar />

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
              <span>{user?.name || user?.username || "Guest"}</span>
              <ChevronDown size={16} />
            </button>

            {profileOpen && (
              <div className="dropdown-menu">
                <button onClick={toggleTheme}>
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
