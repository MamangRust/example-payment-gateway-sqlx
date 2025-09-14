import { TableMerchant } from "@/components/admin/merchant/table";
import useListMerchant from "@/hooks/admin/merchant/ListMerchant";

export default function MerchantPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetMerchants,
    pagination,
    showModal,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
  } = useListMerchant();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableMerchant
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            loadingGetMerchants={loadingGetMerchants}
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
