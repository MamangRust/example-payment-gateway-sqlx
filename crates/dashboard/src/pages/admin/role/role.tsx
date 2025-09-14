import useListRole from "@/hooks/admin/role/ListRole";
import { TableRole } from "@/components/admin/role";

export default function RolePage() {
  const {
    table,
    search,
    setSearch,
    loadingGetRoles,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  } = useListRole();
  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableRole
            search={search}
            setSearch={setSearch}
            showModal={showModal}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetRoles={loadingGetRoles}
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
