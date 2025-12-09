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
import useModalUserTrashed from "@/store/user/trashed/modal";
import useUserTrashedStore from "@/store/user/trashed/trashed";
import { userTrashedColumns } from "@/components/admin/user";
import { FindAllUserTrashed } from "@/types/domain/request";

export default function useListUserTrashed() {
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
    useModalUserTrashed();

  const {
    users,
    pagination,
    loadingGetUsersTrashed,
    setErrorGetUsersTrashed,
    setLoadingGetUsersTrashed,
    findAllUsersTrashed,
  } = useUserTrashedStore();

  const table = useReactTable({
    data: users || [],
    columns: userTrashedColumns,
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
  }, [loadingGetUsersTrashed]);

  useEffect(() => {
    const fetchUsers = async () => {
      try {
        setLoadingGetUsersTrashed(true);

        const searchReq: FindAllUserTrashed = {
          search: search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findAllUsersTrashed(searchReq);
      } catch (error: any) {
        setErrorGetUsersTrashed(error);
      } finally {
        setLoadingGetUsersTrashed(false);
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
    loadingGetUsersTrashed,
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
