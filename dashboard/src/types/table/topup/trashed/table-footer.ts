import { Table } from "@tanstack/react-table";
import { Pagination } from "@/types/table";
import { TopupTrashed } from "@/types/model";

export interface TableFooterTopupTrashedProps {
  table: Table<TopupTrashed>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
