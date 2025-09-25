import "../style/users-page.css";
import UsersPaginatedList from "./components/UsersPaginatedList";
import PaginationControls from "./components/PaginationControls";
import { usePagination } from "../hooks/usePagination";
import PaginationFilters from "./components/PaginationFilters";

export default function UsersPage() {
  const pagination = usePagination();

  return (
    <div className="users-page">
      <PaginationFilters
        paginationState={pagination.paginationState}
        setFilters={pagination.setFilters}
      />

      <div className="users-list-container">
        <UsersPaginatedList
          paginationState={pagination.paginationState}
          updateFromResponse={pagination.updateFromResponse}
          exclude_self={false}
          status={2}
        />
      </div>

      <div className="users-pagination">
        <PaginationControls
          paginationState={pagination.paginationState}
          onPrev={pagination.onPrev}
          onNext={pagination.onNext}
        />
      </div>
    </div>
  );
}
