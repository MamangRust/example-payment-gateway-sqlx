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
import useModalSaldoTrashed from "@/store/saldo/trashed/modal";
import useSaldoTrashedStore from "@/store/saldo/trashed/trashed";
import { saldoTrashedColumns } from "@/components/admin/saldo";
import { FindAllSaldoTrashed } from "@/types/domain/request";

export default function useListSaldoTrashed() {
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
    useModalSaldoTrashed();

  const {
    saldos,
    pagination,
    loadingGetSaldosTrashed,
    setErrorGetSaldosTrashed,
    setLoadingGetSaldosTrashed,
    findAllSaldosTrashed,
  } = useSaldoTrashedStore();

  const table = useReactTable({
    data: saldos || [],
    columns: saldoTrashedColumns,
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
  }, [loadingGetSaldosTrashed]);

  useEffect(() => {
    const fetchUsers = async () => {
      try {
        setLoadingGetSaldosTrashed(true);

        const searchReq: FindAllSaldoTrashed = {
          search: search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findAllSaldosTrashed(searchReq);
      } catch (error: any) {
        setErrorGetSaldosTrashed(error);
      } finally {
        setLoadingGetSaldosTrashed(false);
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
    loadingGetSaldosTrashed,
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
