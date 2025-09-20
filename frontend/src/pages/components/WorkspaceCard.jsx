import "../../style/cardgrid.css";
import { useNavigate } from "react-router-dom";
import { Users, RotateCcw, Calendar, Image } from "lucide-react";
import { TimeAgo } from "../../utils/TimeAgo";
import { Endpoint } from "../../utils/EndPoints";

export default function WorkspaceCard({ workspace }) {
    const navigate = useNavigate();

    const handleOpenWorkspace = async (id) => {
        navigate(`${Endpoint.workspace}/${id}`)
    }
    
    function Avatar({ image_url }) {
        if (image_url) {
            return <img src={image_url} alt="Workspace avatar" />
        } else {
            return <div className="avatar-icon">
                <Image size={20} />
            </div>
        }
    }

    function Title({ title, created_at, updated_at }) {
        return <div className="card-title-container">
            <div className="card-title">{title}</div>
            <div className="card-dates">
                <div className="created">
                    <Calendar size={14} />
                    <span>{new Date(created_at).toLocaleDateString()}</span>
                </div>
                <div className="updated">
                    <RotateCcw size={14} />
                    <span>{TimeAgo(new Date(updated_at))}</span>
                </div>
            </div>
        </div>
    }

    function MemberCount({ member_count }) {
        return <div className="members-count">
            <Users size={16} />
            <span>{member_count}</span>
        </div>
    }

    return (
        <div className="card">
            <div className="card-header">
                <Avatar image_url={workspace.image_url} />
                <Title
                    title={workspace.name}
                    created_at={workspace.created_at}
                    updated_at={workspace.updated_at}
                />
                <MemberCount member_count={workspace.member_count} />
            </div>

            {/* Description */}
            <div className="card-description" title={workspace.description}>
                {workspace.description || "No description"}
            </div>

            {/* Button */}
            <button className="btn-primary" onClick={() => handleOpenWorkspace(workspace.id)}>
                Open Workspace
            </button>
        </div>
    );
}
