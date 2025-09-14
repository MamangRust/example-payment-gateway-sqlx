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
import useModalWithdrawTrashed from "@/store/withdraw/trashed/modal";
import useWithdrawTrashedStore from "@/store/withdraw/trashed/trashed";

import { FindAllWithdrawTrashed } from "@/types/domain/request";
import { useToast } from "@/hooks/use-toast";
import { withdrawTrashedColumns } from "@/components/admin/withdraw";

export default function useListWithdrawTrashed() {
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
    useModalWithdrawTrashed();

  const {
    withdraws,
    pagination,
    loadingGetWithdrawsTrashed,
    setErrorGetWithdrawsTrashed,
    setLoadingGetWithdrawsTrashed,
    findAllWithdrawsTrashed,
  } = useWithdrawTrashedStore();

  const table = useReactTable({
    data: withdraws || [],
    columns: withdrawTrashedColumns,
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
  }, [loadingGetWithdrawsTrashed]);

  useEffect(() => {
    const fetchUsers = async () => {
      try {
        setLoadingGetWithdrawsTrashed(true);

        const searchReq: FindAllWithdrawTrashed = {
          search: search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findAllWithdrawsTrashed(searchReq);
      } catch (error: any) {
        setErrorGetWithdrawsTrashed(error);
      } finally {
        setLoadingGetWithdrawsTrashed(false);
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
    loadingGetWithdrawsTrashed,
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
