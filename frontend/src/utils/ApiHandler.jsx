// API url will be extracted from the environment
const API_URL = import.meta.env.VITE_API_URL;
const USER_ENDPOINT = "/user";

export const User = {
    me: () => dispatcher.get(`${USER_ENDPOINT}/me`),
    login: (credentials) => dispatcher.post(`${USER_ENDPOINT}/login`, credentials, { form: true }),
    logout: () => dispatcher.post(`${USER_ENDPOINT}/logout`),
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
    return response.data;
}
