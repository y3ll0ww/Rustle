
import DashboardPage from "../pages/DashboardPage.jsx";
import LoginPage from "../pages/LoginPage.jsx";
import WorkspacePage from '../pages/WorkspacePage.jsx';
import WorkspaceListPage from '../pages/WorkspacesListPage.jsx';
import UsersPage from "../pages/UsersPage.jsx";
import Loading from "../components/Loading.jsx";

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
    { title: "Login", path: Endpoint.login, element: <LoginPage /> },
]

export const sharedRoutes = [
    { title: "Loading Preview", path: "/loading-preview", element: <Loading /> },
]

export const protectedRoutes = [
    { title: "Dashboard", path: Endpoint.dashboard, element: <DashboardPage /> },
    { title: "Workspaces", path: Endpoint.workspaces, element: <WorkspaceListPage /> },
    { title: "Users", path: Endpoint.users, element: <UsersPage /> },
    { title: "Workspace", path: `${Endpoint.workspace}/:id`, element: <WorkspacePage /> },
    { title: "User", path: `${Endpoint.users}/:id`, element: <div>Users</div> },
];
