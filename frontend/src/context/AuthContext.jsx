import { createContext, useContext, useEffect, useState } from "react";
import { User } from "../utils/ApiHandler";

const AuthContext = createContext();
export const useAuth = () => useContext(AuthContext);

export const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(true);

  // Check if session exists on mount
  useEffect(() => {
    const checkSession = async () => {
      try {
        const data = await User.me();
        setUser(data);
      } catch (err) {
        setUser(null);
      } finally {
        setLoading(false);
      }
    };

    checkSession();
  }, []);

  // Checks if a user is stored in the JWT guard.
  // Should be triggerd after login, but can be checked at any time.
  const check_session = async () => {
    const user = await User.me();
    setUser(user);
  };

  // Logout (optional endpoint on backend)
  const logout = async () => {
    try {
      await User.logout();
    } catch (err) {
      console.error("Login error:", err);
      alert("Login failed.");
    }
    setUser(null);
  };

  return (
    <AuthContext.Provider value={{ user, loading, check_session, logout }}>
      {children}
    </AuthContext.Provider>
  );
};
