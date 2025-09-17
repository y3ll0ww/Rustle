import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import App from "./App.jsx";
import LoginPage from "./pages/LoginPage.jsx";
import './index.css'
import { AuthProvider } from "./context/AuthContext.jsx";
import ProtectedRoute from "./context/ProtectedRoute.jsx";
import PublicRoute from "./context/PublicRoute.jsx";
import DashboardPage from "./pages/DashboardPage.jsx";
import { Endpoint } from "./utils/EndPoints.jsx";
import LoadingPage from "./pages/LoadingPage.jsx";

const homeRoute = <Route
  path={Endpoint.home}
  element={
    <ProtectedRoute fallback={<App />}>
      <DashboardPage />
    </ProtectedRoute>
  }
/>;

const publicRoutes = [
  { path: Endpoint.login, element: <LoginPage /> },
]

const sharedRoutes = [
  { path: "/loading-preview", element: <LoadingPage /> },
]

const protectedRoutes = [
  { path: Endpoint.dashboard, element: <DashboardPage /> }
];

export default function SharedRoute({ children }) {
  return children;
}

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <AuthProvider>
      <BrowserRouter>
        <Routes>
          {homeRoute}

          {publicRoutes.map((route) => (
            <Route
              key={route.path}
              path={route.path}
              element={<PublicRoute>{route.element}</PublicRoute>}
            />
          ))}

          {sharedRoutes.map((route) => (
            <Route
              key={route.path}
              path={route.path}
              element={route.element}
            />
          ))}

          {protectedRoutes.map((route) => (
            <Route
              key={route.path}
              path={route.path}
              element={<ProtectedRoute>{route.element}</ProtectedRoute>}
            />
          ))}
        </Routes>
      </BrowserRouter>
    </AuthProvider>
  </React.StrictMode>
);
