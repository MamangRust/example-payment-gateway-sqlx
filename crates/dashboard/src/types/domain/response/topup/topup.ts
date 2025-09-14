import {
  Topup,
  TopupMonthAmount,
  TopupMonthMethod,
  TopupMonthStatusFailed,
  TopupMonthStatusSuccess,
  TopupTrashed,
  TopupYearlyAmount,
  TopupYearlyMethod,
  TopupYearStatusFailed,
  TopupYearStatusSuccess,
} from "@/types/model";
import { PaginationMeta } from "../pagination";

export interface ApiResponseTopupMonthStatusSuccess {
  status: string;
  message: string;
  data: TopupMonthStatusSuccess[];
}

export interface ApiResponseTopupYearStatusSuccess {
  status: string;
  message: string;
  data: TopupYearStatusSuccess[];
}

export interface ApiResponseTopupMonthStatusFailed {
  status: string;
  message: string;
  data: TopupMonthStatusFailed[];
}

export interface ApiResponseTopupYearStatusFailed {
  status: string;
  message: string;
  data: TopupYearStatusFailed[];
}

export interface ApiResponseTopupMonthMethod {
  status: string;
  message: string;
  data: TopupMonthMethod[];
}

export interface ApiResponseTopupYearMethod {
  status: string;
  message: string;
  data: TopupYearlyMethod[];
}

export interface ApiResponseTopupMonthAmount {
  status: string;
  message: string;
  data: TopupMonthAmount[];
}

export interface ApiResponseTopupYearAmount {
  status: string;
  message: string;
  data: TopupYearlyAmount[];
}

export interface ApiResponseTopup {
  status: string;
  message: string;
  data?: Topup;
}

export interface ApiResponsesTopup {
  status: string;
  message: string;
  data: Topup[];
}

export interface ApiResponsePaginationTopup {
  status: string;
  message: string;
  data: Topup[];
  pagination: PaginationMeta;
}

export interface ApiResponsePaginationTopupDeleteAt {
  status: string;
  message: string;
  data: TopupTrashed[];
  pagination: PaginationMeta;
}

export interface ApiResponseTopupDelete {
  status: string;
  message: string;
}

export interface ApiResponseTopupAll {
  status: string;
  message: string;
}
