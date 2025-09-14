import {
  Transfer,
  TransferMonthAmount,
  TransferMonthStatusFailed,
  TransferMonthStatusSuccess,
  TransferTrashed,
  TransferYearAmount,
  TransferYearStatusFailed,
  TransferYearStatusSuccess,
} from "@/types/model";
import { PaginationMeta } from "../pagination";

export interface ApiResponseTransferMonthStatusSuccess {
  status: string;
  message: string;
  data: TransferMonthStatusSuccess[];
}

export interface ApiResponseTransferYearStatusSuccess {
  status: string;
  message: string;
  data: TransferYearStatusSuccess[];
}

export interface ApiResponseTransferMonthStatusFailed {
  status: string;
  message: string;
  data: TransferMonthStatusFailed[];
}

export interface ApiResponseTransferYearStatusFailed {
  status: string;
  message: string;
  data: TransferYearStatusFailed[];
}

export interface ApiResponseTransferMonthAmount {
  status: string;
  message: string;
  data: TransferMonthAmount[];
}

export interface ApiResponseTransferYearAmount {
  status: string;
  message: string;
  data: TransferYearAmount[];
}

export interface ApiResponseTransfer {
  status: string;
  message: string;
  data?: Transfer;
}

export interface ApiResponseTransfers {
  status: string;
  message: string;
  data: Transfer[];
}

export interface ApiResponseTransferDelete {
  status: string;
  message: string;
}

export interface ApiResponseTransferAll {
  status: string;
  message: string;
}

export interface ApiResponsePaginationTransfer {
  status: string;
  message: string;
  data: Transfer[];
  pagination: PaginationMeta;
}

export interface ApiResponsePaginationTransferDeleteAt {
  status: string;
  message: string;
  data: TransferTrashed[];
  pagination: PaginationMeta;
}
