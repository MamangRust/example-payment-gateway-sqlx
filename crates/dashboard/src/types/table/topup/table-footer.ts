import { Table } from "@tanstack/react-table";
import { Pagination } from "../pagination";
import { Topup } from "@/types/model";

export interface TableFooterTopupProps {
  table: Table<Topup>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
