import { Table } from "@tanstack/react-table";
import { Pagination } from "@/types/table/pagination";
import { CardTrashed } from "@/types/model/card";

export interface TableFooterCardTrashedProps {
  table: Table<CardTrashed>;
  pagination: Pagination;
  onPageChange: (page: number) => void;
  onPageSizeChange: (pageSize: number) => void;
}
