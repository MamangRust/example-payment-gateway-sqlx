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
import { topupColumns } from "@/components/admin/topup/table";
import useTopupStore from "@/store/topup/topup";
import useModalTopup from "@/store/topup/modal";
import { FindAllTopup } from "@/types/domain/request/topup";
import { useToast } from "@/hooks/use-toast";

export default function useListTopup() {
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = useState({});
  const [search, setSearch] = useState("");
  const [isLoadingWithDelay, setIsLoadingWithDelay] = useState(false);
  const [pageSize, setPageSize] = useState(10);
  const [currentPage, setCurrentPage] = useState(1);

  const {
    topups,
    pagination,
    loadingGetTopups,
    setLoadingGetTopups,
    findAllTopups,
  } = useTopupStore();
  const { toast } = useToast();

  const { showModal } = useModalTopup();

  const table = useReactTable({
    data: topups || [],
    columns: topupColumns,
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
  }, [loadingGetTopups]);

  useEffect(() => {
    const fetchTopups = async () => {
      try {
        setLoadingGetTopups(true);

        const searchReq: FindAllTopup = {
          search: search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findAllTopups(searchReq);
      } catch (error) {
        console.error("Error fetching users:", error);
      } finally {
        setLoadingGetTopups(false);
      }
    };

    fetchTopups();
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
    loadingGetTopups,
    currentPage,
    pageSize,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  };
}
