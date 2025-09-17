import { Navigate } from "react-router-dom";
import { useAuth } from "./AuthContext";
import { Endpoint } from "../utils/EndPoints";
import LoadingPage from "../pages/LoadingPage";

const ProtectedRoute = ({ fallback, children }) => {
  const { user, loading } = useAuth();

  if (loading) return <LoadingPage />;
  if (!user) {
    return fallback ? fallback : <Navigate to={Endpoint.login} replace />;
  }

  return children;
};

export default ProtectedRoute;
