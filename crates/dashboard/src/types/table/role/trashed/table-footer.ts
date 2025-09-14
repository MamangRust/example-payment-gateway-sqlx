import { RoleTrashed } from "@/types/model/role";
import { Table } from "@tanstack/react-table";
import { Pagination } from "@/types/table";

export interface TableFooterRoleTrashedProps {
  table: Table<RoleTrashed>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
