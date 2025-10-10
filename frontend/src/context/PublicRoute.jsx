import { Navigate } from "react-router-dom";
import { useAuth } from "./AuthContext";
import Loading from "../components/Loading";

const PublicRoute = ({ children }) => {
  const { user, loading } = useAuth();

  if (loading) return <Loading size="large" />;
  if (user) return <Navigate to="/" replace />;

  return children;
};

export default PublicRoute;
