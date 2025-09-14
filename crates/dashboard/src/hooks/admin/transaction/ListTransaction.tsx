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
import { transactionColumns } from "@/components/admin/transaction/table";
import useTransactionStore from "@/store/transaction/transaction";
import useModalTransaction from "@/store/transaction/modal";
import { FindAllTransaction } from "@/types/domain/request";
import { useToast } from "@/hooks/use-toast";

export default function useListTransaction() {
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = useState({});
  const [search, setSearch] = useState("");
  const [isLoadingWithDelay, setIsLoadingWithDelay] = useState(false);
  const [pageSize, setPageSize] = useState(10);
  const [currentPage, setCurrentPage] = useState(1);

  const {
    transactions,
    pagination,
    loadingGetTransactions,
    setLoadingGetTransactions,
    findAllTransactions,
  } = useTransactionStore();
  const { showModal } = useModalTransaction();
  const { toast } = useToast();

  const table = useReactTable({
    data: transactions || [],
    columns: transactionColumns,
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
  }, [loadingGetTransactions]);

  useEffect(() => {
    const fetchTransactions = async () => {
      try {
        setLoadingGetTransactions(true);

        const searchReq: FindAllTransaction = {
          search: search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findAllTransactions(searchReq);
      } catch (error) {
        console.error("Error fetching users:", error);
      } finally {
        setLoadingGetTransactions(false);
      }
    };

    fetchTransactions();
  }, [search, currentPage, pageSize]);

  const handlePageChange = (newPage: number) => {
    setCurrentPage(newPage);
  };

  const handlePageSizeChange = (newSize: number) => {
    console.log("Changing page size to:", newSize);
    setPageSize(newSize);
    setCurrentPage(1);
  };

  return {
    table,
    search,
    setSearch,
    loadingGetTransactions,
    currentPage,
    pageSize,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  };
}
