import { Navigate } from "react-router-dom";
import { useAuth } from "./AuthContext";
import Loading from "../components/Loading";
import { usePageTitle } from "../hooks/usePageTitle";

const PublicRoute = ({ title, children }) => {
  const { user, loading } = useAuth();

  usePageTitle(title);

  if (loading) return <Loading size="large" />;
  if (user) return <Navigate to="/" replace />;

  return children;
};

export default PublicRoute;
