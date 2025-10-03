import './index.css'
import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import App from "./App.jsx";
import { AuthProvider } from "./context/AuthContext.jsx";
import ProtectedRoute from "./context/ProtectedRoute.jsx";
import PublicRoute from "./context/PublicRoute.jsx";
import DashboardPage from "./pages/DashboardPage.jsx";
import { Endpoint, publicRoutes, sharedRoutes, protectedRoutes } from "./utils/EndPoints.jsx";
import DashboardLayout from "./layouts/DashboardLayout.jsx";
import ConditionalLayout from "./layouts/ConditionalLayout.jsx";
import { ThemeProvider } from "./context/ThemeContext.jsx";
import { AlertProvider } from './context/AlertContext.jsx';

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

export default function SharedRoute({ children }) {
  return children;
}

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <AuthProvider>
      <ThemeProvider>
        <AlertProvider>
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
        </AlertProvider>
      </ThemeProvider>
    </AuthProvider>
  </React.StrictMode >
);
