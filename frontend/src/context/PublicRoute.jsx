import { Navigate } from "react-router-dom";
import { useAuth } from "./AuthContext";
import LoadingPage from "../pages/LoadingPage";

const PublicRoute = ({ children }) => {
  const { user, loading } = useAuth();

  if (loading) return <LoadingPage />;
  if (user) return <Navigate to="/" replace />;

  return children;
};

export default PublicRoute;
