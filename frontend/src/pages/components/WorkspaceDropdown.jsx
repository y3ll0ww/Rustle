import { PackageIcon, PackagePlus, Pencil, Trash, UserRoundPlusIcon, UsersRoundIcon } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";
import { Endpoint } from "../../utils/EndPoints";

export default function WorkspaceDropDown({ handleEdit, handleDelete, handleShowUsers }) {
    const ListItem = ({ icon: Icon, text, onClick, ...props }) => {
        return <button onClick={onClick} {...props}>
            <Icon size={22} />
            <span>{text}</span>
        </button>
    }

    return <div className="dropdown-menu">
        <ListItem
            icon={Pencil}
            text="Edit workspace"
            onClick={handleEdit}
        />

        <div className="dropdown-divider" />

        <ListItem
            icon={PackageIcon}
            text="Show projects"
        />
        <ListItem
            icon={PackagePlus}
            text="Add new project"
        />

        <div className="dropdown-divider" />

        <ListItem
            icon={UsersRoundIcon}
            text="Show members"
            onClick={handleShowUsers}
        />
        <ListItem
            icon={UserRoundPlusIcon}
            text="Add new member"
        />

        <div className="dropdown-divider" />

        <ListItem
            icon={Trash}
            text="Delete workspace"
            style={{ color: "var(--error)" }}
            onClick={handleDelete}
        />
    </div>
};