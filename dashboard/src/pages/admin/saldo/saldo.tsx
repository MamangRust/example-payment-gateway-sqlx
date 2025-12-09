import { TableSaldo } from "@/components/admin/saldo";
import useListSaldo from "@/hooks/admin/saldo/ListSaldo";

export default function SaldoPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetSaldos,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  } = useListSaldo();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableSaldo
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            showModal={showModal}
            loadingGetCards={loadingGetSaldos}
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
