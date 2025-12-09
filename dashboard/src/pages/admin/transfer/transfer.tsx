import { TableTransfer } from "@/components/admin/transfer/table";
import useListTransfer from "@/hooks/admin/transfer/ListTransfer";

export default function TransferPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetTransfers,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  } = useListTransfer();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableTransfer
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetTransfers={loadingGetTransfers}
            table={table}
            showModal={showModal}
            pagination={pagination}
            handlePageChange={handlePageChange}
            handlePageSizeChange={handlePageSizeChange}
          />
        </div>
      </main>
    </div>
  );
}
