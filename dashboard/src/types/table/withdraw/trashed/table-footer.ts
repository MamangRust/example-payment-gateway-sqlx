import { Table } from "@tanstack/react-table";
import { Pagination } from "@/types/table";
import { WithdrawTrashed } from "@/types/model";

export interface TableFooterWithdrawTrashedProps {
  table: Table<WithdrawTrashed>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
