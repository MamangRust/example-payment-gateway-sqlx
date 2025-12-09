import { Table } from "@tanstack/react-table";

import { MerchantTransaction } from "@/types/model";
import { Pagination } from "../../pagination";

export interface TableFooterMerchantTransactionProps {
  table: Table<MerchantTransaction>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
