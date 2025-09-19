import "../../style/topbar.css";
import { useEffect, useState, useRef } from "react";
import { ChevronDown, LogOut, Search, SunMoon } from "lucide-react";
import { useAuth } from "../../context/AuthContext";
import { useTheme } from "../../context/ThemeContext";

export default function Topbar() {
  const { user, logout } = useAuth();
  const { toggleTheme } = useTheme();
  const [profileOpen, setProfileOpen] = useState(false);
  const dropdownRef = useRef(null);

  useEffect(() => {
    function handleClickOutside(e) {
      if (dropdownRef.current && !dropdownRef.current.contains(e.target)) {
        setProfileOpen(false);
      }
    }

    function handleEscape(e) {
      if (e.key === "Escape") setProfileOpen(false);
    }
  
    if (profileOpen) {
      document.addEventListener("mousedown", handleClickOutside);
      document.addEventListener("keydown", handleEscape);
    } else {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleEscape);
    }
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleEscape);
    }
  }, [profileOpen]);

  function Searchbar() {
    return <div className="search-bar">
      <Search size={18} />
      <input type="text" placeholder="Search..." />
    </div>
  }

  function ProfileButton() {
    return <button
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
  }

  function DropdownMenu() {
    return <div className="dropdown-menu">
      <button onClick={toggleTheme}>
        <SunMoon size={22} />
        <span>Toggle Theme</span>
      </button>
      <button onClick={logout}>
        <LogOut size={22} />
        <span>Logout</span>
      </button>
    </div>
  }

  return (
    <header className="topbar">
      <Searchbar />
      <div className="profile-dropdown" ref={dropdownRef}>
        <ProfileButton />
        {profileOpen && <DropdownMenu />}
      </div>
    </header>
  );
}
