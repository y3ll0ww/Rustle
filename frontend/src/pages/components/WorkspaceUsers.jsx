import "../../style/page-users.css";
import { usePagination } from "../../hooks/usePagination";
import UsersPaginatedList from "./UsersPaginatedList";
import PaginationControls from "./PaginationControls";
import PaginationFilters from "./PaginationFilters";
import { useParams } from "react-router-dom";
import { ArrowBigLeftIcon } from "lucide-react";

export default function WorkspaceUsersPage({ handleBack }) {
  const pagination = usePagination();
  const { id } = useParams();

  return (
    <div className="users-page">
      <div style={{
        display: "flex",
        alignItems: "center",
        gap: "0.5rem",
        width: "100%",
        flexShrink: 0,
      }}
      >
        <button
          className="btn-secondary btn-actions"
          onClick={handleBack}
        >
          <ArrowBigLeftIcon />
        </button>

        <div style={{ flex: 1 }}>
          <PaginationFilters
            paginationState={pagination.paginationState}
            setFilters={pagination.setFilters}
          />
        </div>
      </div>

      <div
        className="users-list-container"
      >
        <UsersPaginatedList
          paginationState={pagination.paginationState}
          updateFromResponse={pagination.updateFromResponse}
          exclude_self={false}
          workspace={id}
        />
      </div>

      <div className="users-pagination">
        <PaginationControls
          paginationState={pagination.paginationState}
          onPrev={pagination.onPrev}
          onNext={pagination.onNext}
        />
      </div>
    </div >
  );
}
