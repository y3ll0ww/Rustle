import "../../style/title.css";
import { TimeAgo } from "../../utils/TimeAgo";
import { BrainCircuitIcon, Calendar, RotateCcw } from "lucide-react";

export default function Title({
    name,
    image_url,
    onChange,
    created_at,
    updated_at,
    isEditing,
}) {
    const Image = () => {
        if (!!image_url) {
            return <img
                src={image_url}
                alt="Workspace image"
                className="avatar medium"
                style={{ marginTop: "5px" }}
            />
        } else {
            return <div className="avatar medium placeholder">
                <BrainCircuitIcon size={30}/>
            </div>
        }
    }

    return (
        <div style={{ display: "flex", alignItems: "center", gap: "1rem" }}>
            <div style={{ marginLeft: "1rem" }}>
                <Image />
            </div>

            <div style={{ display: "flex", flexDirection: "column", justifyContent: "center" }}>
                {isEditing ? (
                    <input
                        type="text"
                        className="page-title-input"
                        onChange={(e) => onChange(e.target.value)}
                        defaultValue={name}
                    />
                ) : (
                    <h1 style={{ margin: "0 0 0.25rem 0" }}>{name}</h1>
                )}

                <div className="dates" style={{ display: "flex", gap: "1rem" }}>
                    <div className="created" style={{ display: "flex", alignItems: "center", gap: "0.25rem" }}>
                        <Calendar size={20} />
                        <span>{new Date(created_at).toLocaleDateString()}</span>
                    </div>
                    <div className="updated" style={{ display: "flex", alignItems: "center", gap: "0.25rem" }}>
                        <RotateCcw size={20} />
                        <span>{TimeAgo(new Date(updated_at))}</span>
                    </div>
                </div>
            </div>
        </div>
    );
}