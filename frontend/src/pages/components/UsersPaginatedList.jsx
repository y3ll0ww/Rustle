import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { User } from "../../utils/ApiHandler";
import NoWorkspacesPage from "../NoWorkspacePage";
import LoadingPage from "../LoadingPage";
import { usePagination } from "../../hooks/usePagination";
import { Endpoint } from "../../utils/EndPoints";
import UserListItem from "./UserListItem";

export default function UsersPaginatedList({ exclude_self, status, role, workspace, pagination }) {
    const navigate = useNavigate();
    const [users, setUsers] = useState([]);
    const [loading, setLoading] = useState(true);
    const { paginationState, updateFromResponse } = usePagination();

    // Function for fetching projects from workspace
    const getUsers = async () => {
        try {
            const data = await User.paginated({ exclude_self, status, role, workspace, pagination });
            //user?exclude_self=false&workspace=ec10c387-a999-470f-a458-b7f1b2f79b13
            updateFromResponse(data);
        } catch {
            updateFromResponse(null);
        } finally {
            setLoading(false);
        }
    };

    // Get users via API
    useEffect(() => {
        // Initial fetch
        getUsers();

        // Poll every 30s
        const interval = setInterval(() => {
            getUsers();
        }, 30000);

        // Cleanup interval on unmount
        return () => clearInterval(interval);
    }, []);

    useEffect(() => {
        setUsers(paginationState?.records || []);
    }, [paginationState])

    // Return loading screen when the workspaces are being fetched
    if (loading) {
        return <div className="flex items-center justify-center h-full">
            <LoadingPage />
        </div>;
    }

    // Return no content page if user is not part of any workspace
    if (users.length === 0) {
        return <NoWorkspacesPage />;
    }

    const handleClickUser = async (id) => {
        navigate(`${Endpoint.users}/${id}`)
    }

    // Return the content of the page
    return <div className="flex w-screen">
        <ul className="member-list">
            {paginationState.records.map((record) => {
                const index = record.index;
                const role = record.data.role;
                const user = record.data.user;

                return <UserListItem
                    key={index}
                    index={index}
                    role={role}
                    user={user}
                    handleClick={() => handleClickUser(user.id)}
                />
            })}
        </ul>
    </div>;
}
