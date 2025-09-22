import "../../style/members.css";
import { User } from "lucide-react";
import { workspaceMemberType, workspaceRoleColor } from "../../utils/RoleInterpreter";

export default function UserListItem({ index, user, role, handleClick }) {
    const displayName =
        user.first_name && user.last_name
            ? `${user.first_name} ${user.last_name}`
            : user.first_name
                ? user.first_name
                : user.last_name
                    ? user.last_name
                    : user.username;

    function Avatar({ url: image_url }) {
        if (image_url) {
            return <img src={image_url} alt={displayName} className="avatar" />
        } else {
            return <div className="avatar placeholder">
                <User size={20} />
            </div>
        }
    }

    function NameRole() {
        return <div className="member-main">
            <span className="member-name">{displayName}</span>
            <span
                className="member-role"
                style={{ color: workspaceRoleColor(role) }}
            >
                {workspaceMemberType(role)}
            </span>
        </div>
    }

    return (
        <li
            key={user.id}
            className="member-item"
            onClick={handleClick}
        >
            <div className="member-index">{index}</div>

            <Avatar url={user.avatar_url} />

            {/* Info */}
            <div className="member-info">
                <NameRole />

                {user.job_title && (
                    <div className="member-job">
                        <Briefcase size={14} /> {user.job_title}
                    </div>
                )}

                {user.phone && (
                    <div className="member-phone">
                        <Phone size={14} /> {user.phone}
                    </div>
                )}

                <div className="member-email">{user.email}</div>
            </div>
        </li>
    );
}
