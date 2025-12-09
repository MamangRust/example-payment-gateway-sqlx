import useListUser from "@/hooks/admin/user/ListUser";
import { TableUser } from "@/components/admin/user";

export default function UserPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetUsers,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  } = useListUser();
  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableUser
            search={search}
            setSearch={setSearch}
            showModal={showModal}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetUsers={loadingGetUsers}
            table={table}
            pagination={pagination}
            handlePageChange={handlePageChange}
            handlePageSizeChange={handlePageSizeChange}
          />
        </div>
      </main>
    </div>
  );
}
