// API url will be extracted from the environment
const API_URL = import.meta.env.VITE_API_URL;
const EP_USER = "/user";
const EP_WORKSPACES = "/workspaces";

export const defaultPagination = {
    page: 1,
    per_page: 20,
    sort_by: "created_at",
    sort_order: "desc",
};

export const User = {
    me: () => dispatcher.get(`${EP_USER}/me`),
    login: (credentials) => dispatcher.post(`${EP_USER}/login`, credentials, { form: true }),
    logout: () => dispatcher.post(`${EP_USER}/logout`),
    paginated: ({ exclude_self, status, role, workspace, paginationState }) => {
        // /user/?<status>&<role>&<workspace>&<exclude_self>
        const params = new URLSearchParams();
        if (status) params.append("status", status);
        if (role) params.append("role", role);
        if (workspace) params.append("workspace", workspace);
        params.append("exclude_self", exclude_self);

        const pagination = {
            page: paginationState?.page || defaultPagination.page,
            limit: paginationState?.limit || defaultPagination.limit,
            search: paginationState?.search || defaultPagination.search,
            sort_by: paginationState?.sortBy || deafaultPagination.sort_by,
            sort_dir: paginationState?.sortOrder || defaultPagination.sort_dir,
        };

        return dispatcher.post(`${EP_USER}?${params.toString()}`, pagination);
    },
};

export const Workspaces = {
    from_user: () => dispatcher.get(`${EP_WORKSPACES}`),
    by_id: (id) => dispatcher.get(`${EP_WORKSPACES}/${id}`),
    new_project: ({ id, project_form }) => dispatcher.post(`${EP_WORKSPACES}/${id}`, project_form, { form: true }),
    update_information: ({ id, updated_info }) => dispatcher.put(`${EP_WORKSPACES}/${id}/update`, updated_info)
}

export const Projects = {
    paginated: ({ workspace, user, pagination }) => {
        const params = new URLSearchParams();
        if (workspace) params.append("workspace", workspace);
        if (user) params.append("user", user);

        return dispatcher.post(`/projects?${params.toString()}`, pagination || defaultPagination);
    },
};

// Convenience helpers
const dispatcher = {
    get: (endpoint, options) => Dispatch(endpoint, { ...options, method: "GET" }),
    post: (endpoint, body, options) => Dispatch(endpoint, { ...options, method: "POST", body }),
    put: (endpoint, body, options) => Dispatch(endpoint, { ...options, method: "PUT", body }),
    del: (endpoint, options) => Dispatch(endpoint, { ...options, method: "DELETE" }),
};

async function Dispatch(endpoint, { method = "GET", body, headers = {}, form = false } = {}) {
    // Create the request configuration
    const config = {
        method,
        headers: { ...headers },
        // Always include cookies
        credentials: "include",
    };

    if (body) {
        if (form) {
            // send as application/x-www-form-urlencoded
            config.headers["Content-Type"] = "application/x-www-form-urlencoded";
            config.body = new URLSearchParams(body).toString();
        } else {
            // send as JSON
            config.headers["Content-Type"] = "application/json";
            config.body = JSON.stringify(body);
        }
    }

    // Make the call and collect the response
    const res = await fetch(`${API_URL}${endpoint}`, config);

    // Extract the response message
    const response = await res.json();

    // Handle error scenario
    if (!res.ok) {
        let message = `Request failed with ${res.status}`;
        try {
            const errorData = await res.json();
            message += errorData.message || message;
        } catch (_) { }
        throw new Error(message);
    }

    // Handle success scenario
    console.log(response.message);
    console.log(response.data);
    return response.data;
}
