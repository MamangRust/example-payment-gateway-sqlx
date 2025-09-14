import { TableMerchantTrashed } from "@/components/admin/merchant";
import useListMerchantTrashed from "@/hooks/admin/merchant/trashed/ListTrashed";

export default function MerchantTrashedPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetMerchantsTrashed,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModalRestoreAll,
    showModalDeletePermanentAll,
  } = useListMerchantTrashed();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableMerchantTrashed
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetMerchantsTrashed={loadingGetMerchantsTrashed}
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
