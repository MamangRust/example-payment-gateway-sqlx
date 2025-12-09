import {
  Withdraw,
  WithdrawMonthlyAmount,
  WithdrawMonthStatusFailed,
  WithdrawMonthStatusSuccess,
  WithdrawTrashed,
  WithdrawYearlyAmount,
  WithdrawYearStatusFailed,
  WithdrawYearStatusSuccess,
} from "@/types/model";
import { PaginationMeta } from "../pagination";

export interface ApiResponseWithdrawMonthStatusSuccess {
  status: string;
  message: string;
  data: WithdrawMonthStatusSuccess[];
}

export interface ApiResponseWithdrawYearStatusSuccess {
  status: string;
  message: string;
  data: WithdrawYearStatusSuccess[];
}

export interface ApiResponseWithdrawMonthStatusFailed {
  status: string;
  message: string;
  data: WithdrawMonthStatusFailed[];
}

export interface ApiResponseWithdrawYearStatusFailed {
  status: string;
  message: string;
  data: WithdrawYearStatusFailed[];
}

export interface ApiResponseWithdrawMonthAmount {
  status: string;
  message: string;
  data: WithdrawMonthlyAmount[];
}

export interface ApiResponseWithdrawYearAmount {
  status: string;
  message: string;
  data: WithdrawYearlyAmount[];
}

export interface ApiResponsesWithdraw {
  status: string;
  message: string;
  data: Withdraw[];
}

export interface ApiResponseWithdraw {
  status: string;
  message: string;
  data?: Withdraw;
}

export interface ApiResponseWithdrawDelete {
  status: string;
  message: string;
}

export interface ApiResponseWithdrawAll {
  status: string;
  message: string;
}

export interface ApiResponsePaginationWithdraw {
  status: string;
  message: string;
  data: Withdraw[];
  pagination: PaginationMeta;
}

export interface ApiResponsePaginationWithdrawDeleteAt {
  status: string;
  message: string;
  data: WithdrawTrashed[];
  pagination: PaginationMeta;
}
