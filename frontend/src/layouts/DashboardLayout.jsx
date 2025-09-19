import "../style/dashboard.css";
import { Outlet } from "react-router-dom";
import Sidebar from "./components/Sidebar";
import Topbar from "./components/Topbar";

export default function DashboardLayout() {
  function PageContent() {
    return <main className="page-content">
      <div className="content-wrapper">
        <Outlet />
      </div>
    </main>
  }

  return (
    <div className="dashboard-layout">
      <Sidebar />
      <div className="main-content">
        <Topbar />
        <PageContent />
      </div>
    </div>
  );
}
