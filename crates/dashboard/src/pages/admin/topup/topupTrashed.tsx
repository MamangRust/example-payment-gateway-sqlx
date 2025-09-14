import { TableTopupTrashed } from "@/components/admin/topup";
import useListTopupTrashed from "@/hooks/admin/topup/trashed/ListTrashed";

export default function TopupTrashedPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetTopupsTrashed,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModalRestoreAll,
    showModalDeletePermanentAll,
  } = useListTopupTrashed();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableTopupTrashed
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetTopupsTrashed={loadingGetTopupsTrashed}
            showModalRestoreAll={showModalRestoreAll}
            showModalDeletePermanentAll={showModalDeletePermanentAll}
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
