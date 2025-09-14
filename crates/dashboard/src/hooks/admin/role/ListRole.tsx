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
import { RoleColumns } from "@/components/admin/role/table/table-column";
import useRoleStore from "@/store/role/role";
import useModalRole from "@/store/role/modal";
import { useToast } from "@/hooks/use-toast";
import { FindAllRole } from "@/types/domain/request";

export default function useListRole() {
  const [sorting, setSorting] = useState<SortingState>([]);
  const [columnFilters, setColumnFilters] = useState<ColumnFiltersState>([]);
  const [columnVisibility, setColumnVisibility] = useState<VisibilityState>({});
  const [rowSelection, setRowSelection] = useState({});
  const [search, setSearch] = useState("");
  const [isLoadingWithDelay, setIsLoadingWithDelay] = useState(false);
  const [pageSize, setPageSize] = useState(10);
  const [currentPage, setCurrentPage] = useState(1);

  const { showModal } = useModalRole();
  const { toast } = useToast();

  const {
    Roles,
    pagination,
    loadingGetRoles,
    setLoadingGetRoles,
    findAllRoles,
  } = useRoleStore();

  const table = useReactTable({
    data: Roles || [],
    columns: RoleColumns,
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
  }, [loadingGetRoles]);

  useEffect(() => {
    const fetchRoles = async () => {
      try {
        setLoadingGetRoles(true);

        const searchReq: FindAllRole = {
          search: search,
          page: currentPage,
          page_size: pageSize,
          toast: toast,
        };

        await findAllRoles(searchReq);
      } catch (error) {
        console.error("Error fetching Roles:", error);
      } finally {
        setLoadingGetRoles(false);
      }
    };

    fetchRoles();
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
    loadingGetRoles,
    currentPage,
    pageSize,
    pagination,
    handlePageChange,
    handlePageSizeChange,
    isLoadingWithDelay,
    showModal,
  };
}
