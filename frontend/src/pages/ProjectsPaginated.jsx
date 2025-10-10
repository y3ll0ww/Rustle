import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Projects } from "../utils/ApiHandler";
import { usePagination } from "../hooks/usePagination";
import { Endpoint } from "../utils/EndPoints";
import ProjectListItem from "./components/ProjectListItem";
import { PackagePlusIcon } from "lucide-react";
import Loading from "../components/Loading";

export default function ProjectsPaginatedPage({ workspace_id }) {
    const navigate = useNavigate();
    const [projects, setProjects] = useState([]);
    const [loading, setLoading] = useState(true);
    const { paginationState, updateFromResponse, nextPage, prevPage } = usePagination();

    // Function for fetching projects from workspace
    const getProjectsInWorkspace = async () => {
        try {
            const data = await Projects.paginated({ workspace: workspace_id });
            updateFromResponse(data);
        } catch {
            updateFromResponse(null);
        } finally {
            setLoading(false);
        }
    };

    // Get workspaces projects via API
    useEffect(() => {
        // Initial fetch
        getProjectsInWorkspace();

        // Poll every 30s
        const interval = setInterval(() => {
            getProjectsInWorkspace();
        }, 30000);

        // Cleanup interval on unmount
        return () => clearInterval(interval);
    }, []);

    useEffect(() => {
        setProjects(paginationState?.records || []);
    }, [paginationState])

    // Return loading screen when the workspaces are being fetched
    if (loading) {
        return <div>
            <Loading />
        </div>;
    }

    // Return no content page if user is not part of any workspace
    if (projects.length === 0) {
        return (
            <div className="content-container hoverable">
                <div className="no-records">
                    <PackagePlusIcon style={{ marginRight: "0.5rem" }}/>
                    Add a Project
                </div>
            </div>
        )
    }

    const handleOpenProject = async (id) => {
        navigate(`${Endpoint.workspace}/${id}`)
    }

    // Return the content of the page
    return <div className="flex w-screen">
        {paginationState.records.map((record) => {
            const project = record.data;
            return <ProjectListItem 
                key={project.id}
                index={record.index}
                project={project}
                handleClick={handleOpenProject}
            />
        })}
    </div>;
}
