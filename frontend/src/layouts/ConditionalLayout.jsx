import { Outlet } from "react-router-dom";
import { useAuth } from "../context/AuthContext";
import DashboardLayout from "../layouts/DashboardLayout";

export default function ConditionalLayout({ publicElement }) {
  const { user } = useAuth();

  // If user is logged in; wrap in dashboard
  if (user) {
    return <DashboardLayout><Outlet /></DashboardLayout>;
  }

  // If user is logged out; just render public page
  return publicElement;
}
