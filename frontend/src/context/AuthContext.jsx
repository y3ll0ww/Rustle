import { createContext, useContext, useEffect, useState } from "react";

const AuthContext = createContext();
export const useAuth = () => useContext(AuthContext);

export const AuthProvider = ({ children }) => {
  const [user, setUser] = useState(null);
  const [loading, setLoading] = useState(true);

  // Check if session exists on mount
  useEffect(() => {
    const checkSession = async () => {
      try {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/user/me`, {
          credentials: "include",
        });
        if (res.ok) {
          const data = await res.json();
          setUser(data);
        } else {
          setUser(null);
        }
      } catch (err) {
        console.error("Session check failed:", err);
        setUser(null);
      } finally {
        setLoading(false);
      }
    };

    checkSession();
  }, []);

  // Triggered after login
  const login = async () => {
    // After /user/login succeeds, re-check session
    await new Promise((resolve) => setTimeout(resolve, 200)); // small delay in case cookie not set yet
    const res = await fetch(`${import.meta.env.VITE_API_URL}/user/me`, {
      credentials: "include",
    });
    if (res.ok) {
      const response = await res.json();
      console.log(response.data);
      setUser(response.data);
    }
  };

  // Logout (optional endpoint on backend)
  const logout = async () => {
    try {
      await fetch(`${import.meta.env.VITE_API_URL}/user/logout`, {
        method: "POST",
        credentials: "include",
      });
    } catch {}
    setUser(null);
  };

  return (
    <AuthContext.Provider value={{ user, loading, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};
