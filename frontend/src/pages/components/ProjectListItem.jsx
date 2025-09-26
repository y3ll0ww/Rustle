import "../../style/projects.css";
import { Users, RotateCcw, Calendar, Image } from "lucide-react";
import { TimeAgo } from "../../utils/TimeAgo";

export default function ProjectListItem({ index, project, handleClick }) {
    function Avatar({ image_url }) {
        if (image_url) {
            return <img src={image_url} alt={project.name} className="avatar" />;
        } else {
            return (
                <div className="avatar placeholder">
                    <Image size={20} />
                </div>
            );
        }
    }

    return (
        <li key={project.id} className="project-item" onClick={() => handleClick(project.id)}>
            <div className="project-index">{index}</div>

            <Avatar image_url={project.image_url} />

            <div className="project-info">
                <div className="project-name">{project.name}</div>

                <div className="project-meta">
                    <div className="created">
                        <Calendar size={14} />
                        <span>{new Date(project.created_at).toLocaleDateString()}</span>
                    </div>
                    <div className="updated">
                        <RotateCcw size={14} />
                        <span>{TimeAgo(new Date(project.updated_at))}</span>
                    </div>
                </div>
            </div>

            {/* Right-aligned member count */}
            <div className="project-members">
                <Users size={16} />
                <span>{project.member_count}</span>
            </div>
        </li>
    );
}
