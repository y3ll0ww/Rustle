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

const homeRoute = <Route
  path="/"
  element={
    <ProtectedRoute fallback={<App />}>
      <DashboardPage />
    </ProtectedRoute>
  }
/>;

const publicRoutes = [
  { path: "/login", element: <LoginPage /> },
]

const protectedRoutes = [
  { path: "/dashboard", element: <DashboardPage /> }
];

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
