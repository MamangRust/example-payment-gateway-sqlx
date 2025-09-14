import { TableWithdraw } from "@/components/admin/withdraw/table";
import useListWithdraw from "@/hooks/admin/withdraw/ListWithdraw";

export default function WithdrawPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetWithdraws,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  } = useListWithdraw();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableWithdraw
            search={search}
            setSearch={setSearch}
            showModal={showModal}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetWithdraws={loadingGetWithdraws}
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
