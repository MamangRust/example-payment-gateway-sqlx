import { Table } from "@tanstack/react-table";
import { Pagination } from "@/types/table";
import { MerchantTrashed } from "@/types/model";

export interface TableFooterMerchantTrashedProps {
  table: Table<MerchantTrashed>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
