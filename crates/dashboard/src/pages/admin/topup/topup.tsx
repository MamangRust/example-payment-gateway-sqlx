import { TableTopup } from "@/components/admin/topup";
import useListTopup from "@/hooks/admin/topup/ListTopup";

export default function TopupPage() {
  const {
    table,
    search,

    setSearch,
    loadingGetTopups,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  } = useListTopup();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableTopup
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetTopups={loadingGetTopups}
            showModal={showModal}
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
