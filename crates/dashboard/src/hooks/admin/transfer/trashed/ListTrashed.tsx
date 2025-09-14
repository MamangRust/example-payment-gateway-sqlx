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
import useModalTransaferTrashed from "@/store/transfer/trashed/modal";
import useTransferTrashedStore from "@/store/transfer/trashed/trashed";
import { transferTrashedColumns } from "@/components/admin/transfer";
import { FindAllTransferTrashed } from "@/types/domain/request";

export default function useListTransferTrashed() {
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
    useModalTransaferTrashed();

  const {
    transfers,
    pagination,
    loadingGetTransfersTrashed,
    setErrorGetTransfersTrashed,
    setLoadingGetTransfersTrashed,
    findAllTransfersTrashed,
  } = useTransferTrashedStore();

  const table = useReactTable({
    data: transfers || [],
    columns: transferTrashedColumns,
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
  }, [loadingGetTransfersTrashed]);

  useEffect(() => {
    const fetchUsers = async () => {
      try {
        setLoadingGetTransfersTrashed(true);

        const searchReq: FindAllTransferTrashed = {
          search: search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findAllTransfersTrashed(searchReq);
      } catch (error: any) {
        setErrorGetTransfersTrashed(error);
      } finally {
        setLoadingGetTransfersTrashed(false);
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
    loadingGetTransfersTrashed,
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
