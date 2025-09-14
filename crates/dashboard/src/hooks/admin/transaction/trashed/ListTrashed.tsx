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
import useModalTransactionTrashed from "@/store/transaction/trashed/modal";
import useTransactionTrashedStore from "@/store/transaction/trashed/trashed";
import { FindAllTransactionTrashed } from "@/types/domain/request";
import { transactionTrashedColumns } from "@/components/admin/transaction";

export default function useListTransactionTrashed() {
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = useState({});
  const [search, setSearch] = useState("");
  const [isLoadingWithDelay, setIsLoadingWithDelay] = useState(false);
  const [pageSize, setPageSize] = useState(10);
  const [currentPage, setCurrentPage] = useState(1);

  const { toast } = useToast();
  const { showModalRestoreAll, showModalDeletePermanentAll } =
    useModalTransactionTrashed();

  const {
    transactions,
    pagination,
    loadingGetTransactionsTrashed,
    setErrorGetTransactionsTrashed,
    setLoadingGetTransactionsTrashed,
    findAllTransactionsTrashed,
  } = useTransactionTrashedStore();

  const table = useReactTable({
    data: transactions || [],
    columns: transactionTrashedColumns,
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
  }, [loadingGetTransactionsTrashed]);

  useEffect(() => {
    const fetchUsers = async () => {
      try {
        setLoadingGetTransactionsTrashed(true);

        const searchReq: FindAllTransactionTrashed = {
          search: search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findAllTransactionsTrashed(searchReq);
      } catch (error: any) {
        setErrorGetTransactionsTrashed(error);
      } finally {
        setLoadingGetTransactionsTrashed(false);
      }
    };

    fetchUsers();
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
    loadingGetTransactionsTrashed,
    currentPage,
    pageSize,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModalRestoreAll,
    showModalDeletePermanentAll,
  };
}
