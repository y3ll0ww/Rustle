import UsersPaginatedList from "./components/UsersPaginatedList";
import PaginationControls from "./components/PaginationControls";
import { usePagination } from "../hooks/usePagination";
import PaginationFilters from "./components/PaginationFilters";

export default function UsersPage() {
  const pagination = usePagination();

  // Return the content of the page
  return <div className="flex w-screen">
    <h1>Users</h1>

    <PaginationFilters paginationState={pagination.paginationState} />
    
    <UsersPaginatedList
        paginationState={pagination.paginationState}
        updateFromResponse={pagination.updateFromResponse}
        exclude_self={false}
        status={2}
    />

    <PaginationControls
        paginationState={pagination.paginationState}
        onPrev={pagination.onPrev}
        onNext={pagination.onNext}
    />

  </div>;
}
