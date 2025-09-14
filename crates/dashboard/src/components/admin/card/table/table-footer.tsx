import { Button } from "@/components/ui/button";
import PaginationDropdown from "@/components/admin/dropdown-pagination";
import { TableFooterCardProps } from "@/types/table/card";

const TableFooterCard = ({
  table,
  pagination,
  onPageChange,
  onPageSizeChange,
}: TableFooterCardProps) => {
  const safePagination = {
    currentPage: pagination?.currentPage || 1,
    pageSize: pagination?.pageSize || 10,
    totalItems: pagination?.totalItems || 0,
    totalPages: pagination?.totalPages || 1,
  };

  const totalPages = safePagination.totalPages;
  const pageWindowSize = 5;

  const startPage =
    Math.floor((safePagination.currentPage - 1) / pageWindowSize) *
    pageWindowSize;
  const endPage = Math.min(startPage + pageWindowSize, totalPages);

  const handleNextPage = () => {
    if (safePagination.currentPage < totalPages) {
      onPageChange(safePagination.currentPage + 1);
    }
  };

  const handlePreviousPage = () => {
    if (safePagination.currentPage > 1) {
      onPageChange(safePagination.currentPage - 1);
    }
  };

  const handlePageClick = (page: number) => {
    onPageChange(page + 1);
  };

  return (
    <div className="flex justify-between items-center w-full">
      <div className="text-sm text-gray-500">
        Showing {safePagination.pageSize * (safePagination.currentPage - 1) + 1}{" "}
        to{" "}
        {Math.min(
          safePagination.pageSize * safePagination.currentPage,
          safePagination.totalItems,
        )}{" "}
        of {safePagination.totalItems} entries
      </div>

      <PaginationDropdown
        pagination={safePagination}
        setPagination={onPageSizeChange}
        table={table}
      />
      <div className="flex items-center space-x-4">
        <div className="flex items-center space-x-2">
          <Button
            variant="outline"
            onClick={handlePreviousPage}
            disabled={safePagination.currentPage === 1}
          >
            Previous
          </Button>

          {[...Array(endPage - startPage).keys()].map((_, i) => {
            const page = startPage + i;
            return (
              <Button
                key={page}
                variant={
                  page + 1 === safePagination.currentPage
                    ? "default"
                    : "outline"
                }
                onClick={() => handlePageClick(page)}
              >
                {page + 1}
              </Button>
            );
          })}

          {endPage < totalPages && <span>...</span>}

          <Button
            variant="outline"
            onClick={handleNextPage}
            disabled={safePagination.currentPage === totalPages}
          >
            Next
          </Button>
        </div>
      </div>
    </div>
  );
};

export default TableFooterCard;
