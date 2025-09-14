import { TableSaldoTrashed } from "@/components/admin/saldo";
import useListSaldoTrashed from "@/hooks/admin/saldo/trashed/ListTrashed";

export default function SaldoTrashedPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetSaldosTrashed,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModalRestoreAll,
    showModalDeletePermanentAll,
  } = useListSaldoTrashed();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableSaldoTrashed
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetSaldosTrashed={loadingGetSaldosTrashed}
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
