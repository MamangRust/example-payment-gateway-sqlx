import { Table } from "@tanstack/react-table";
import { Pagination } from "../pagination";
import { Card } from "@/types/model/card";

export interface TableFooterCardProps {
  table: Table<Card>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
