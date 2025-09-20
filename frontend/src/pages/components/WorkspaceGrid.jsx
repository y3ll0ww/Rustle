import "../../style/cardgrid.css";
import { useEffect, useState } from "react";
import { Workspaces } from "../../utils/ApiHandler";
import { Users, RotateCcw, Calendar, Image as ImageIcon } from "lucide-react";

function timeAgo(date) {
  const now = new Date();
  const diff = now - new Date(date); // difference in ms

  const seconds = Math.floor(diff / 1000);
  const minutes = Math.floor(seconds / 60);
  const hours = Math.floor(minutes / 60);
  const days = Math.floor(hours / 24);
  const months = Math.floor(days / 30);
  const years = Math.floor(days / 365);

  if (years > 0) return `${years} year${years > 1 ? 's' : ''} ago`;
  if (months > 0) return `${months} month${months > 1 ? 's' : ''} ago`;
  if (days > 0) return `${days} day${days > 1 ? 's' : ''} ago`;
  if (hours > 0) return `${hours} hour${hours > 1 ? 's' : ''} ago`;
  if (minutes > 0) return `${minutes} minute${minutes > 1 ? 's' : ''} ago`;
  return `${seconds} second${seconds !== 1 ? 's' : ''} ago`;
}

export default function WorkspaceGrid({ workspaces }) {

    function WorkspaceCard({ workspace }) {
        return (
            <div className="card">
                {/* Header: Avatar + Title + Members */}
                <div className="card-header">
                    {/* Avatar */}
                    {workspace.image_url ? (
                        <img src={workspace.image_url} alt="workspace avatar" />
                    ) : (
                        <div className="avatar-icon">
                            <ImageIcon size={20} />
                        </div>
                    )}

                    {/* Title + Dates */}
                    <div className="card-title-container">
                        <div className="card-title">{workspace.name}</div>

                        <div className="card-dates">
                            <div className="created">
                                <Calendar size={14} />
                                <span>{new Date(workspace.created_at).toLocaleDateString()}</span>
                            </div>
                            <div className="updated">
                                <RotateCcw size={14} />
                                <span>{timeAgo(new Date(workspace.updated_at))}</span>
                            </div>
                        </div>
                    </div>

                    {/* Members count */}
                    <div className="members-count">
                        <Users size={16} />
                        <span>{workspace.member_count}</span>
                    </div>
                </div>

                {/* Description */}
                <div className="card-description" title={workspace.description}>
                    {workspace.description || "No description"}
                </div>

                {/* Button */}
                <button className="btn-primary" onClick={() => console.log("Open workspace", workspace.id)}>
                    Open Workspace
                </button>
            </div>
        );
    }


    // Page content
    return (
        <div>
            <h2 style={{ marginLeft: "1.5rem"}}>Workspaces</h2>
            <div className="card-grid">
                {workspaces.map((workspace) => (
                    <WorkspaceCard workspace={workspace} key={workspace.id} />
                ))}
            </div>
        </div>
    );
}