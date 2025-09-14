import { Table } from "@tanstack/react-table";
import { Pagination } from "../pagination";
import { Transfer } from "@/types/model";

export interface TableFooterTransferProps {
  table: Table<Transfer>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
