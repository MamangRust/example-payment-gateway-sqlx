import { User } from "@/types/model/user";
import { Table } from "@tanstack/react-table";
import { Pagination } from "../pagination";

export interface TableFooterUserProps {
  table: Table<User>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
