import { useEffect, useState } from "react";
import {
  ColumnFiltersState,
  getCoreRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  SortingState,
  useReactTable,
  VisibilityState,
} from "@tanstack/react-table";
import { withdrawColumns } from "@/components/admin/withdraw/table";
import useWithdrawStore from "@/store/withdraw/withdraw";
import useModalWithdraw from "@/store/withdraw/modal";
import { useToast } from "@/hooks/use-toast";
import { FindByCardNumberWithdraw } from "@/types/domain/request";

export default function useListWithdrawCardNumber({
  card_number,
}: {
  card_number: string;
}) {
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = useState({});
  const [search, setSearch] = useState("");
  const [isLoadingWithDelay, setIsLoadingWithDelay] = useState(false);
  const [pageSize, setPageSize] = useState(10);
  const [currentPage, setCurrentPage] = useState(1);

  const {
    withdraws,
    pagination,
    loadingGetCardNumberWithdraw,
    setLoadingGetCardNumberWithdraw,
    findByCardNumberWithdraw,
  } = useWithdrawStore();
  const { showModal } = useModalWithdraw();
  const { toast } = useToast();

  const table = useReactTable({
    data: withdraws || [],
    columns: withdrawColumns,
    onSortingChange: setSorting,
    onColumnFiltersChange: setColumnFilters,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    onColumnVisibilityChange: setColumnVisibility,
    onRowSelectionChange: setRowSelection,
    state: {
      sorting,
      columnFilters,
      columnVisibility,
      rowSelection,
      pagination: {
        pageIndex: currentPage - 1,
        pageSize: pageSize,
      },
    },
    pageCount: pagination.totalPages || 1,
    manualPagination: true,
  });

  useEffect(() => {
    setIsLoadingWithDelay(true);

    const delayTimer = setTimeout(() => {
      setIsLoadingWithDelay(false);
    }, 2000);

    return () => clearTimeout(delayTimer);
  }, [loadingGetCardNumberWithdraw]);

  useEffect(() => {
    const fetchWithdraws = async () => {
      try {
        setLoadingGetCardNumberWithdraw(true);

        const searchReq: FindByCardNumberWithdraw = {
          cardNumber: card_number,
          search: search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findByCardNumberWithdraw(searchReq);
      } catch (error) {
        console.error("Error fetching users:", error);
      } finally {
        setLoadingGetCardNumberWithdraw(false);
      }
    };

    fetchWithdraws();
  }, [search, currentPage, pageSize]);

  const handlePageChange = (newPage: number) => {
    setCurrentPage(newPage);
  };

  const handlePageSizeChange = (newSize: number) => {
    setPageSize(newSize);
    setCurrentPage(1);
  };

  return {
    table,
    search,
    setSearch,
    loadingGetCardNumberWithdraw,
    currentPage,
    pageSize,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  };
}
