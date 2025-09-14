import {
  Saldo,
  SaldoMonthBalance,
  SaldoMonthTotalBalance,
  SaldoTrashed,
  SaldoYearBalance,
  SaldoYearTotalBalance,
} from "@/types/model";
import { PaginationMeta } from "../pagination";

export interface ApiResponseSaldo {
  status: string;
  message: string;
  data: Saldo;
}

export interface ApiResponsesSaldo {
  status: string;
  message: string;
  data: Saldo[];
}

export interface ApiResponseSaldoDelete {
  status: string;
  message: string;
}

export interface ApiResponseSaldoAll {
  status: string;
  message: string;
}

export interface ApiResponseMonthTotalSaldo {
  status: string;
  message: string;
  data: SaldoMonthTotalBalance[];
}

export interface ApiResponseYearTotalSaldo {
  status: string;
  message: string;
  data: SaldoYearTotalBalance[];
}

export interface ApiResponseMonthSaldoBalances {
  status: string;
  message: string;
  data: SaldoMonthBalance[];
}

export interface ApiResponseYearSaldoBalances {
  status: string;
  message: string;
  data: SaldoYearBalance[];
}

export interface ApiResponsePaginationSaldo {
  status: string;
  message: string;
  data: Saldo[];
  pagination: PaginationMeta;
}

export interface ApiResponsePaginationSaldoDeleteAt {
  status: string;
  message: string;
  data: SaldoTrashed[];
  pagination: PaginationMeta;
}
