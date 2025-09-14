import { Table } from "@tanstack/react-table";
import { Pagination } from "../pagination";
import { Transaction } from "@/types/model";

export interface TableFooterTransactionProps {
  table: Table<Transaction>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
