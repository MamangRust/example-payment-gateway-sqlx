import { TableTransactionTrashed } from "@/components/admin/transaction";
import useListTransactionTrashed from "@/hooks/admin/transaction/trashed/ListTrashed";

export default function TransactionTrashedPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetTransactionsTrashed,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModalRestoreAll,
    showModalDeletePermanentAll,
  } = useListTransactionTrashed();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableTransactionTrashed
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetTransactionsTrashed={loadingGetTransactionsTrashed}
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
