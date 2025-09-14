import { TableWithdrawTrashed } from "@/components/admin/withdraw";
import useListWithdrawTrashed from "@/hooks/admin/withdraw/trashed/ListTrashed";

export default function WithdrawTrashedPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetWithdrawsTrashed,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModalRestoreAll,
    showModalDeletePermanentAll,
  } = useListWithdrawTrashed();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableWithdrawTrashed
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetWithdrawsTrashed={loadingGetWithdrawsTrashed}
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
