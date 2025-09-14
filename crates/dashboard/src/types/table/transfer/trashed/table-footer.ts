import { Table } from "@tanstack/react-table";
import { Pagination } from "@/types/table";
import { TransferTrashed } from "@/types/model";

export interface TableFooterTransferTrashedProps {
  table: Table<TransferTrashed>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
