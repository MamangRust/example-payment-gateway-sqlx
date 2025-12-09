import useListCard from "@/hooks/admin/card/ListCard";
import TableCard from "@/components/admin/card/table/table-card";

export default function CardPage() {
  const {
    table,
    search,
    setSearch,
    loadingGetCards,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  } = useListCard();

  return (
    <div className="flex h-full overflow-hidden">
      <main className="flex-1 p-6 overflow-auto pb-20">
        <div className="flex-1 flex flex-col min-h-0">
          <TableCard
            search={search}
            setSearch={setSearch}
            isLoadingWithDelay={isLoadingWithDelay}
            showModal={showModal}
            loadingGetCards={loadingGetCards}
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
