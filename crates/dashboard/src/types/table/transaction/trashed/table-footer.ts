import { Table } from "@tanstack/react-table";
import { Pagination } from "@/types/table";
import { TransactionTrashed } from "@/types/model";

export interface TableFooterTransactionTrashedProps {
  table: Table<TransactionTrashed>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
