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
import useModalMerchantTrashed from "@/store/merchant/trashed/modal";
import { useToast } from "@/hooks/use-toast";
import useMerchantTrashedStore from "@/store/merchant/trashed/trashed";
import { merchantTrashedColumns } from "@/components/admin/merchant";
import { FindAllMerchantTrashed } from "@/types/domain/request";

export default function useListMerchantTrashed() {
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
    useModalMerchantTrashed();

  const {
    merchants,
    pagination,
    loadingGetMerchantsTrashed,
    setLoadingGetMerchantsTrashed,
    setErrorGetMerchantsTrashed,
    findAllMerchantsTrashed,
  } = useMerchantTrashedStore();

  const table = useReactTable({
    data: merchants || [],
    columns: merchantTrashedColumns,
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
  }, [loadingGetMerchantsTrashed]);

  useEffect(() => {
    const fetchUsers = async () => {
      try {
        setLoadingGetMerchantsTrashed(true);

        const searchReq: FindAllMerchantTrashed = {
          search: search,
          page: currentPage,
          pageSize: pageSize,
          toast: toast,
        };

        await findAllMerchantsTrashed(searchReq);
      } catch (error: any) {
        setErrorGetMerchantsTrashed(error);
      } finally {
        setLoadingGetMerchantsTrashed(false);
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
    loadingGetMerchantsTrashed,
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
