import { UserTrashed } from "@/types/model/user";
import { Table } from "@tanstack/react-table";
import { Pagination } from "@/types/table";

export interface TableFooterUserTrashedProps {
  table: Table<UserTrashed>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
