import { Table } from "@tanstack/react-table";
import { Pagination } from "../pagination";
import { Merchant } from "@/types/model";

export interface TableFooterMerchantProps {
  table: Table<Merchant>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
