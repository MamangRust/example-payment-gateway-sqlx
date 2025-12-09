import { Table } from "@tanstack/react-table";
import { Pagination } from "../pagination";
import { Saldo } from "@/types/model";

export interface TableFooterSaldoProps {
  table: Table<Saldo>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
