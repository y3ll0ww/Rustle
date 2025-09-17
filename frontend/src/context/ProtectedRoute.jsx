import { Navigate } from "react-router-dom";
import { useAuth } from "./AuthContext";

const ProtectedRoute = ({ fallback, children }) => {
  const { user, loading } = useAuth();

  if (loading) return <p>Loading...</p>;
  if (!user) {
    return fallback ? fallback : <Navigate to="/login" replace />;
  }

  return children;
};

export default ProtectedRoute;
