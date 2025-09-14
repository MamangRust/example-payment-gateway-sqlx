import {
  Card,
  CardTrashed,
  CardMonthBalance,
  CardYearlyBalance,
  CardMonthTopupAmount,
  CardYearlyTopupAmount,
  CardMonthTransactionAmount,
  CardYearlyTransactionAmount,
  CardMonthTransferAmount,
  CardYearlyTransferAmount,
  DashboardCard,
  DashboardCardCardNumber,
  CardMonthWithdrawAmount,
  CardYearlyWithdrawAmount,
} from "@/types/model";
import { PaginationMeta } from "../pagination";

export interface ApiResponseCard {
  status: string;
  message: string;
  data?: Card;
}

export interface ApiResponseCardDelete {
  status: string;
  message: string;
}

export interface ApiResponseCardAll {
  status: string;
  message: string;
}

export interface ApiResponsePaginationCard {
  status: string;
  message: string;
  data?: Card[];
  pagination?: PaginationMeta;
}

export interface ApiResponsePaginationCardDeleteAt {
  status: string;
  message: string;
  data?: CardTrashed[];
  pagination?: PaginationMeta;
}

export interface ApiResponseMonthlyBalance {
  status: string;
  message: string;
  data?: CardMonthBalance[];
}

export interface ApiResponseYearlyBalance {
  status: string;
  message: string;
  data?: CardYearlyBalance[];
}

export interface ApiResponseDashboardCard {
  status: string;
  message: string;
  data?: DashboardCard;
}

export interface ApiResponseDashboardCardNumber {
  status: string;
  message: string;
  data?: DashboardCardCardNumber;
}

export interface ApiResponseMonthlyTopupAmount {
  status: string;
  message: string;
  data?: CardMonthTopupAmount[];
}

export interface ApiResponseYearlyTopupAmount {
  status: string;
  message: string;
  data?: CardYearlyTopupAmount[];
}

export interface ApiResponseMonthlyTransactionAmount {
  status: string;
  message: string;
  data?: CardMonthTransactionAmount[];
}

export interface ApiResponseYearlyTransactionAmount {
  status: string;
  message: string;
  data?: CardYearlyTransactionAmount[];
}

export interface ApiResponseMonthlyTransferAmount {
  status: string;
  message: string;
  data?: CardMonthTransferAmount[];
}

export interface ApiResponseYearlyTransferAmount {
  status: string;
  message: string;
  data?: CardYearlyTransferAmount[];
}

export interface ApiResponseMonthlyWithdrawAmount {
  status: string;
  message: string;
  data?: CardMonthWithdrawAmount[];
}

export interface ApiResponseYearlyWithdrawAmount {
  status: string;
  message: string;
  data?: CardYearlyWithdrawAmount[];
}
