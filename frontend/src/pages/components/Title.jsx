import "../../style/title.css";
import { TimeAgo } from "../../utils/TimeAgo";
import { Calendar, RotateCcw } from "lucide-react";

export default function Title({
    name,
    onChange,
    created_at,
    updated_at,
    isEditing,
}) {
    return (
        <div>
            {isEditing
                ? <input
                    type="text"
                    className="page-title-input"
                    onChange={(e) => onChange(e.target.value)}
                    defaultValue={name}
                />
                : <h1 style={{ margin: "1rem 1rem 1rem" }}>{name}</h1>
            }

            <div className="dates" style={{ marginLeft: "1rem" }}>
                <div className="created">
                    <Calendar size={20} />
                    <span>{new Date(created_at).toLocaleDateString()}</span>
                </div>
                <div className="updated">
                    <RotateCcw size={20} />
                    <span>{TimeAgo(new Date(updated_at))}</span>
                </div>
            </div>
        </div>
    );
}