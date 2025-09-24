
import DashboardPage from "../pages/DashboardPage.jsx";
import LoginPage from "../pages/LoginPage.jsx";
import LoadingPage from "../pages/LoadingPage.jsx";
import WorkspacePage from '../pages/WorkspacePage.jsx';
import WorkspaceListPage from '../pages/WorkspacesListPage.jsx';
import UsersPage from "../pages/UsersPage.jsx";

export const Endpoint = {
    home: "/",
    login: "/login",
    logout: "/logout",
    dashboard: "/dashboard",
    workspaces: "/workspaces",
    workspace: "/workspace",
    users: "/users",
};

export const publicRoutes = [
    { path: Endpoint.login, element: <LoginPage /> },
]

export const sharedRoutes = [
    { path: "/loading-preview", element: <LoadingPage /> },
]

export const protectedRoutes = [
    { path: Endpoint.dashboard, element: <DashboardPage /> },
    { path: Endpoint.workspaces, element: <WorkspaceListPage /> },
    { path: Endpoint.users, element: <UsersPage /> },
    { path: `${Endpoint.workspace}/:id`, element: <WorkspacePage /> },
    { path: `${Endpoint.users}/:id`, element: <div>Users</div> },
];
