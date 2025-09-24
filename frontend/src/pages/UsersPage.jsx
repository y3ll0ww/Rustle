import UsersPaginatedList from "./components/UsersPaginatedList";
import PaginationControls from "./components/PaginationControls";
import { usePagination } from "../hooks/usePagination";

export default function UsersPage() {
  const { paginationState, onPrev, onNext } = usePagination();

  // Return the content of the page
  return <div className="flex w-screen">
    <h1>Users</h1>
    
    <UsersPaginatedList
        exclude_self={false}
        status={2}
    />

    <PaginationControls
        pagination={paginationState}
        onPrev={onPrev}
        onNext={onNext}
    />

  </div>;
}
