import { TableTransaction } from "@/components/admin/transaction/table";
import useListTransaction from "@/hooks/admin/transaction/ListTransaction";

export default function TransactionPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetTransactions,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  } = useListTransaction();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableTransaction
            search={search}
            setSearch={setSearch}
            showModal={showModal}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetTransactions={loadingGetTransactions}
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
