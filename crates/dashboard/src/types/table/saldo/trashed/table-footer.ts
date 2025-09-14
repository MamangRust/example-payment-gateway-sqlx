import { Table } from "@tanstack/react-table";
import { Pagination } from "@/types/table";
import { SaldoTrashed } from "@/types/model";

export interface TableFooterSaldoTrashedProps {
  table: Table<SaldoTrashed>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
