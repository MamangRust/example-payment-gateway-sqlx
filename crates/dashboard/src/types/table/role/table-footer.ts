import { Role } from "@/types/model/role";
import { Table } from "@tanstack/react-table";
import { Pagination } from "../pagination";

export interface TableFooterRoleProps {
  table: Table<Role>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
