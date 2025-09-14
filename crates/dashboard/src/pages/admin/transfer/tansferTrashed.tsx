import { TableTransferTrashed } from "@/components/admin/transfer";
import useListTransferTrashed from "@/hooks/admin/transfer/trashed/ListTrashed";

export default function TransferTrashedPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetTransfersTrashed,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModalRestoreAll,
    showModalDeletePermanentAll,
  } = useListTransferTrashed();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableTransferTrashed
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetTransfersTrashed={loadingGetTransfersTrashed}
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
