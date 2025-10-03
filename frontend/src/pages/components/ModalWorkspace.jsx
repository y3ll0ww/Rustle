import { PackageIcon, PackagePlus, Pencil, Trash, UserRoundPlusIcon, UsersRoundIcon } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useParams } from "react-router-dom";
import { useAlert } from "../../context/AlertContext";
import { Workspaces } from "../../utils/ApiHandler";
import { Endpoint } from "../../utils/EndPoints";

export default function WorkspaceDropDown({ handleEdit }) {
    const { id } = useParams();
    const { alertSuccess, alertError } = useAlert();
    const navigate = useNavigate();

    const ListItem = ({ icon: Icon, text, onClick, ...props }) => {
        return <button onClick={onClick} {...props}>
            <Icon size={22} />
            <span>{text}</span>
        </button>
    }

    const handleDelete = async (e) => {
        e.preventDefault();

        try {
            await Workspaces.delete(id);
            navigate(Endpoint.workspaces);
            alertSuccess("Workspace deleted successfully!")
        } catch (err) {
            alertError("Error deleting workspace", "Lorem ipsum was conceived as filler text, formatted in a certain way to enable the presentation of graphic elements in documents, without the need for formal ...");
        }
    };

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