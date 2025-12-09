import {
  Transaction,
  TransactionMonthAmount,
  TransactionMonthMethod,
  TransactionMonthStatusFailed,
  TransactionMonthStatusSuccess,
  TransactionTrashed,
  TransactionYearlyAmount,
  TransactionYearMethod,
  TransactionYearStatusFailed,
  TransactionYearStatusSuccess,
} from "@/types/model";
import { PaginationMeta } from "../pagination";

export interface ApiResponseTransactionMonthStatusSuccess {
  status: string;
  message: string;
  data: TransactionMonthStatusSuccess[];
}

export interface ApiResponseTransactionYearStatusSuccess {
  status: string;
  message: string;
  data: TransactionYearStatusSuccess[];
}

export interface ApiResponseTransactionMonthStatusFailed {
  status: string;
  message: string;
  data: TransactionMonthStatusFailed[];
}

export interface ApiResponseTransactionYearStatusFailed {
  status: string;
  message: string;
  data: TransactionYearStatusFailed[];
}

export interface ApiResponseTransactionMonthMethod {
  status: string;
  message: string;
  data: TransactionMonthMethod[];
}

export interface ApiResponseTransactionYearMethod {
  status: string;
  message: string;
  data: TransactionYearMethod[];
}

export interface ApiResponseTransactionMonthAmount {
  status: string;
  message: string;
  data: TransactionMonthAmount[];
}

export interface ApiResponseTransactionYearAmount {
  status: string;
  message: string;
  data: TransactionYearlyAmount[];
}

export interface ApiResponseTransaction {
  status: string;
  message: string;
  data?: Transaction;
}

export interface ApiResponseTransactions {
  status: string;
  message: string;
  data: Transaction[];
}

export interface ApiResponseTransactionDelete {
  status: string;
  message: string;
}

export interface ApiResponseTransactionAll {
  status: string;
  message: string;
}

export interface ApiResponsePaginationTransaction {
  status: string;
  message: string;
  data: Transaction[];
  pagination: PaginationMeta;
}

export interface ApiResponsePaginationTransactionDeleteAt {
  status: string;
  message: string;
  data: TransactionTrashed[];
  pagination: PaginationMeta;
}
