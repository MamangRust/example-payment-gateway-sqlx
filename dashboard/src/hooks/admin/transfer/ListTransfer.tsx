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
import { transferColumns } from "@/components/admin/transfer/table";
import useTransferStore from "@/store/transfer/transfer";
import useModalTransfer from "@/store/transfer/modal";
import { FindAllTransfer } from "@/types/domain/request";
import { useToast } from "@/hooks/use-toast";

export default function useListTransfer({
  card_number = "",
}: {
  card_number?: string;
} = {}) {
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = useState({});
  const [search, setSearch] = useState("");
  const [isLoadingWithDelay, setIsLoadingWithDelay] = useState(false);
  const [pageSize, setPageSize] = useState(10);
  const [currentPage, setCurrentPage] = useState(1);
  const { showModal } = useModalTransfer();

  const {
    transfers,
    pagination,
    loadingGetTransfers,
    setLoadingGetTransfers,
    findAllTransfers,
  } = useTransferStore();
  const { toast } = useToast();

  const table = useReactTable({
    data: transfers || [],
    columns: transferColumns,
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
  }, [loadingGetTransfers]);

  useEffect(() => {
    const fetchTransfers = async () => {
      try {
        setLoadingGetTransfers(true);

        const searchReq: FindAllTransfer = {
          search: card_number ? card_number : search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findAllTransfers(searchReq);
      } catch (error) {
        console.error("Error fetching transfers:", error);
      } finally {
        setLoadingGetTransfers(false);
      }
    };

    fetchTransfers();
  }, [search, currentPage, pageSize, card_number]);

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
    loadingGetTransfers,
    currentPage,
    pageSize,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  };
}
