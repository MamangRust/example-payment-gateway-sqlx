import {
  CreateWithdraw,
  FindByCardNumberWithdraw,
  FindByIdWithdraw,
  TrashedWithdraw,
  UpdateWithdraw,
} from "@/types/domain/request";
import { FindAllWithdraw } from "@/types/domain/request/withdraw/list";
import {
  Withdraw,
  WithdrawMonthlyAmount,
  WithdrawMonthStatusFailed,
  WithdrawMonthStatusSuccess,
  WithdrawYearlyAmount,
  WithdrawYearStatusFailed,
  WithdrawYearStatusSuccess,
} from "@/types/model/withdraw";

export interface WithdrawStore {
  withdraws: Withdraw[] | null;
  withdraw: Withdraw | null;

  monthStatusSuccess: WithdrawMonthStatusSuccess[] | null;
  yearStatusSuccess: WithdrawYearStatusSuccess[] | null;

  monthStatusFailed: WithdrawMonthStatusFailed[] | null;
  yearStatusFailed: WithdrawYearStatusFailed[] | null;

  monthWithdrawAmount: WithdrawMonthlyAmount[] | null;
  yearWithdrawAmount: WithdrawYearlyAmount[] | null;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingMonthStatusSuccess: boolean;
  loadingYearStatusSuccess: boolean;
  loadingMonthStatusFailed: boolean;
  loadingYearStatusFailed: boolean;
  loadingMonthWithdrawAmount: boolean;
  loadingYearWithdrawAmount: boolean;

  loadingGetWithdraws: boolean;
  loadingGetWithdraw: boolean;
  loadingGetCardNumberWithdraw: boolean;
  loadingGetActiveWithdraw: boolean;
  loadingGetTrashedWithdraw: boolean;

  loadingCreateWithdraw: boolean;
  loadingUpdateWithdraw: boolean;
  loadingTrashedWithdraw: boolean;

  // Error states

  errorMonthStatusSuccess: string | null;
  errorYearStatusSuccess: string | null;
  errorMonthStatusFailed: string | null;
  errorYearStatusFailed: string | null;
  errorMonthWithdrawAmount: string | null;
  errorYearWithdrawAmount: string | null;

  errorGetWithdraws: string | null;
  errorGetWithdraw: string | null;
  errorGetCardNumberWithdraw: string | null;
  errorGetActiveWithdraw: string | null;
  errorGetTrashedWithdraw: string | null;

  errorCreateWithdraw: string | null;
  errorUpdateWithdraw: string | null;
  errorTrashedWithdraw: string | null;

  setLoadingMonthStatusSuccess: (value: boolean) => void;
  setLoadingYearStatusSuccess: (value: boolean) => void;
  setLoadingMonthStatusFailed: (value: boolean) => void;
  setLoadingYearStatusFailed: (value: boolean) => void;
  setLoadingMonthWithdrawAmount: (value: boolean) => void;
  setLoadingYearWithdrawAmount: (value: boolean) => void;

  setLoadingGetWithdraws: (value: boolean) => void;
  setLoadingGetWithdraw: (value: boolean) => void;
  setLoadingGetCardNumberWithdraw: (value: boolean) => void;
  setLoadingGetActiveWithdraw: (value: boolean) => void;
  setLoadingGetTrashedWithdraw: (value: boolean) => void;

  setLoadingCreateWithdraw: (value: boolean) => void;
  setLoadingUpdateWithdraw: (value: boolean) => void;
  setLoadingTrashedWithdraw: (value: boolean) => void;

  setErrorMonthStatusSuccess: (value: string | null) => void;
  setErrorYearStatusSuccess: (value: string | null) => void;
  setErrorMonthStatusFailed: (value: string | null) => void;
  setErrorYearStatusFailed: (value: string | null) => void;
  setErrorMonthWithdrawAmount: (value: string | null) => void;
  setErrorYearWithdrawAmount: (value: string | null) => void;

  setErrorGetWithdraws: (value: string | null) => void;
  setErrorGetWithdraw: (value: string | null) => void;
  setErrorGetCardNumberWithdraw: (value: string | null) => void;
  setErrorGetActiveWithdraw: (value: string | null) => void;
  setErrorGetTrashedWithdraw: (value: string | null) => void;

  setErrorCreateWithdraw: (value: string | null) => void;
  setErrorUpdateWithdraw: (value: string | null) => void;
  setErrorTrashedWithdraw: (value: string | null) => void;

  findMonthStatusSuccess: (
    toast: any,
    year: number,
    month: number,
  ) => Promise<void>;
  findYearStatusSuccess: (toast: any, year: number) => Promise<void>;
  findMonthStatusFailed: (
    toast: any,
    year: number,
    month: number,
  ) => Promise<void>;
  findYearStatusFailed: (toast: any, year: number) => Promise<void>;

  findMonthStatusSuccessByCardNumber: (
    toast: any,
    year: number,
    month: number,
    card_number: string,
  ) => Promise<void>;
  findYearStatusSuccessByCardNumber: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;
  findMonthStatusFailedByCardNumber: (
    toast: any,
    year: number,
    month: number,
    card_number: string,
  ) => Promise<void>;
  findYearStatusFailedByCardNumber: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;

  findMonthWithdrawAmount: (toast: any, year: number) => Promise<void>;
  findYearWithdrawAmount: (toast: any, year: number) => Promise<void>;

  findMonthWithdrawAmountByCard: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;
  findYearWithdrawAmountByCard: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;

  findAllWithdraws: (req: FindAllWithdraw) => Promise<void>;
  findByIdWithdraw: (req: FindByIdWithdraw) => Promise<void>;
  findByCardNumberWithdraw: (req: FindByCardNumberWithdraw) => Promise<void>;
  findByActiveWithdraw: (req: FindAllWithdraw) => Promise<void>;
  createWithdraw: (req: CreateWithdraw) => Promise<boolean>;
  updateWithdraw: (req: UpdateWithdraw) => Promise<boolean>;
  trashedWithdraw: (req: TrashedWithdraw) => Promise<boolean>;
}
