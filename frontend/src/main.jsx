import './index.css'
import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import App from "./App.jsx";
import LoginPage from "./pages/LoginPage.jsx";
import { AuthProvider } from "./context/AuthContext.jsx";
import ProtectedRoute from "./context/ProtectedRoute.jsx";
import PublicRoute from "./context/PublicRoute.jsx";
import DashboardPage from "./pages/DashboardPage.jsx";
import { Endpoint } from "./utils/EndPoints.jsx";
import LoadingPage from "./pages/LoadingPage.jsx";
import DashboardLayout from "./layouts/DashboardLayout.jsx";
import ConditionalLayout from "./layouts/ConditionalLayout.jsx";
import { ThemeProvider } from "./context/ThemeContext.jsx";

const homeRoute = <Route
  path={Endpoint.home}
  element={<ConditionalLayout publicElement={<PublicRoute><App /></PublicRoute>} />}
>
  {/* Child route for protected content */}
  <Route
    index
    element={<ProtectedRoute><DashboardPage /></ProtectedRoute>}
  />
</Route>;

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
      <ThemeProvider>
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

            <Route element={<DashboardLayout />}>
              {protectedRoutes.map((route) => (
                <Route
                  key={route.path}
                  path={route.path}
                  element={<ProtectedRoute>{route.element}</ProtectedRoute>}
                />
              ))}
            </Route>

          </Routes>
        </BrowserRouter>
      </ThemeProvider>
    </AuthProvider>
  </React.StrictMode >
);
