import "../../style/cardgrid.css";
import { useEffect, useState } from "react";
import { Workspaces } from "../../utils/ApiHandler";
import { Users, Clock, Calendar, Image as ImageIcon, FileText } from "lucide-react";

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
                                <Calendar size={14} /> {new Date(workspace.created_at).toLocaleDateString()}
                            </div>
                            <div className="updated">
                                <Calendar size={14} /> {new Date(workspace.updated_at).toLocaleDateString()}
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