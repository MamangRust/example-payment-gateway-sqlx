import {
  MerchantMonthlyPaymentMethod,
  MerchantYearlyPaymentMethod,
  MerchantMonthlyAmount,
  MerchantYearlyAmount,
  MerchantMonthlyTotalAmount,
  MerchantYearlyTotalAmount,
  Merchant,
  MerchantTrashed,
  MerchantTransaction,
} from "@/types/model";
import { PaginationMeta } from "../pagination";

export interface ApiResponseMerchantMonthlyPaymentMethod {
  status: string;
  message: string;
  data: MerchantMonthlyPaymentMethod[];
}

export interface ApiResponseMerchantYearlyPaymentMethod {
  status: string;
  message: string;
  data: MerchantYearlyPaymentMethod[];
}

export interface ApiResponseMerchantMonthlyAmount {
  status: string;
  message: string;
  data: MerchantMonthlyAmount[];
}

export interface ApiResponseMerchantYearlyAmount {
  status: string;
  message: string;
  data: MerchantYearlyAmount[];
}

export interface ApiResponseMerchantMonthlyTotalAmount {
  status: string;
  message: string;
  data: MerchantMonthlyTotalAmount[];
}

export interface ApiResponseMerchantYearlyTotalAmount {
  status: string;
  message: string;
  data: MerchantYearlyTotalAmount[];
}

export interface ApiResponsesMerchant {
  status: string;
  message: string;
  data: Merchant[];
}

export interface ApiResponseMerchant {
  status: string;
  message: string;
  data: Merchant;
}

export interface ApiResponseMerchantDelete {
  status: string;
  message: string;
}

export interface ApiResponseMerchantAll {
  status: string;
  message: string;
}

export interface ApiResponsePaginationMerchant {
  status: string;
  message: string;
  data: Merchant[];
  pagination: PaginationMeta;
}

export interface ApiResponsePaginationMerchantDeleteAt {
  status: string;
  message: string;
  data: MerchantTrashed[];
  pagination: PaginationMeta;
}

export interface ApiResponsePaginationMerchantTransaction {
  status: string;
  message: string;
  data: MerchantTransaction[];
  pagination: PaginationMeta;
}
