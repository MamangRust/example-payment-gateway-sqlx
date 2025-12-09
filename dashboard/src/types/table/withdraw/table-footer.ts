import { Table } from "@tanstack/react-table";
import { Pagination } from "../pagination";
import { Withdraw } from "@/types/model";

export interface TableFooterWithdrawProps {
  table: Table<Withdraw>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
