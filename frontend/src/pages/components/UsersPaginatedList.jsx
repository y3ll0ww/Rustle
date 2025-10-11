import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { User } from "../../utils/ApiHandler";
import { Endpoint } from "../../utils/EndPoints";
import UserListItem from "./UserListItem";
import { userRoleColor, userType, workspaceMemberType, workspaceRoleColor } from "../../utils/RoleInterpreter";
import Loading from "../../components/Loading";

export default function UsersPaginatedList({
    paginationState,
    updateFromResponse,
    exclude_self=false,
    status=2,
    role,
    workspace,
}) {
    const navigate = useNavigate();
    const [loading, setLoading] = useState(true);

    // Function for fetching projects from workspace
    const getUsers = async () => {
        try {
            const data = await User.paginated({
                exclude_self,
                status,
                role,
                workspace,
                paginationState
            });
            updateFromResponse(data);
        } catch {
            updateFromResponse(null);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        getUsers();
    }, [
        paginationState.search,
        paginationState.limit,
        paginationState.page,
        paginationState.sortBy,
        paginationState.sortOrder,
    ])

    // Return loading screen when the workspaces are being fetched
    if (loading) {
        return <Loading size="large" />;
    }

    const handleClickUser = async (id) => {
        navigate(`${Endpoint.users}/${id}`)
    }

    // Return the content of the page
    return (
        <div className="flex w-screen">
            {paginationState.records.length > 0 ? (
                <ul className="member-list">
                    {paginationState.records.map((record) => {
                        const index = record.index;
                        const user = record.data.user;
                        const role = record.data.role ?? user.role;

                        const role_type = workspace
                            ? workspaceMemberType(role)
                            : userType(role);

                        const role_color = workspace
                            ? workspaceRoleColor(role)
                            : userRoleColor(role);

                        return (
                            <UserListItem
                                key={index}
                                index={index}
                                user={user}
                                role_type={role_type}
                                role_color={role_color}
                                handleClick={() => handleClickUser(user.id)}
                            />
                        );
                    })}
                </ul>
            ) : (
                <div className="no-records">No records found</div>
            )}
        </div>
    );
}
