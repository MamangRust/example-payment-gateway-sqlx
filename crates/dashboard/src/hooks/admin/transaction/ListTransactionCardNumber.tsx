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
import { useToast } from "@/hooks/use-toast";
import useTransactionStore from "@/store/transaction/transaction";
import { transactionColumns } from "@/components/admin/transaction";
import { FindyByCardNumberTransaction } from "@/types/domain/request";

export default function useListTransactionCardNumber({
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
    transactions,
    pagination,
    loadingGetCardNumberTransaction,
    setLoadingGetCardNumberTransaction,
    findByCardNumberTransaction,
  } = useTransactionStore();

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
  }, [loadingGetCardNumberTransaction]);

  useEffect(() => {
    const fetchTransactions = async () => {
      try {
        setLoadingGetCardNumberTransaction(true);

        const searchReq: FindyByCardNumberTransaction = {
          cardNumber: card_number,
          search: search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findByCardNumberTransaction(searchReq);
      } catch (error) {
        console.error("Error fetching users:", error);
      } finally {
        setLoadingGetCardNumberTransaction(false);
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
    loadingGetCardNumberTransaction,
    currentPage,
    pageSize,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
  };
}
