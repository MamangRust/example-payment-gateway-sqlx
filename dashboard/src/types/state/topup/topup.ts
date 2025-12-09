import {
  CreateTopup,
  FindAllTopup,
  FindByIdTopup,
  TrashedTopup,
  UpdateTopup,
} from "@/types/domain/request/topup";
import {
  Topup,
  TopupMonthAmount,
  TopupMonthMethod,
  TopupMonthStatusFailed,
  TopupMonthStatusSuccess,
  TopupYearlyAmount,
  TopupYearlyMethod,
  TopupYearStatusFailed,
  TopupYearStatusSuccess,
} from "../../model/topup";
import { FindByCardNumberTopup } from "@/types/domain/request/topup/findByCardNumber";

export interface TopupStore {
  topups: Topup[] | null;
  topup: Topup | null;

  monthStatusSuccess: TopupMonthStatusSuccess[] | null;
  yearStatusSuccess: TopupYearStatusSuccess[] | null;

  monthStatusFailed: TopupMonthStatusFailed[] | null;
  yearStatusFailed: TopupYearStatusFailed[] | null;

  monthTopupMethod: TopupMonthMethod[] | null;
  yearTopupMethod: TopupYearlyMethod[] | null;

  monthTopupAmount: TopupMonthAmount[] | null;
  yearTopupAmount: TopupYearlyAmount[] | null;

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

  loadingMonthTopupMethod: boolean;
  loadingYearTopupMethod: boolean;
  loadingMonthTopupAmount: boolean;
  loadingYearTopupAmount: boolean;

  loadingGetTopups: boolean;
  loadingGetTopup: boolean;
  loadingGetActiveTopup: boolean;
  loadingGetTrashedTopup: boolean;
  loadingGetCardNumberTopup: boolean;

  loadingCreateTopup: boolean;
  loadingUpdateTopup: boolean;
  loadingTrashedTopup: boolean;

  errorMonthStatusSuccess: string | null;
  errorYearStatusSuccess: string | null;
  errorMonthStatusFailed: string | null;
  errorYearStatusFailed: string | null;
  errorMonthTopupMethod: string | null;
  errorYearTopupMethod: string | null;
  errorMonthTopupAmount: string | null;
  errorYearTopupAmount: string | null;

  errorGetTopups: string | null;
  errorGetTopup: string | null;
  errorGetActiveTopup: string | null;
  errorGetTrashedTopup: string | null;
  errorGetCardNumberTopup: string | null;

  errorCreateTopup: string | null;
  errorUpdateTopup: string | null;
  errorTrashedTopup: string | null;

  setLoadingMonthStatusSuccess: (value: boolean) => void;
  setLoadingYearStatusSuccess: (value: boolean) => void;
  setLoadingMonthStatusFailed: (value: boolean) => void;
  setLoadingYearStatusFailed: (value: boolean) => void;
  setLoadingMonthTopupMethod: (value: boolean) => void;
  setLoadingYearTopupMethod: (value: boolean) => void;
  setLoadingMonthTopupAmount: (value: boolean) => void;
  setLoadingYearTopupAmount: (value: boolean) => void;

  setLoadingGetTopups: (value: boolean) => void;
  setLoadingGetTopup: (value: boolean) => void;
  setLoadingGetActiveTopup: (value: boolean) => void;
  setLoadingGetTrashedTopup: (value: boolean) => void;
  setLoadingGetCardNumberTopup: (value: boolean) => void;

  setLoadingCreateTopup: (value: boolean) => void;
  setLoadingUpdateTopup: (value: boolean) => void;
  setLoadingTrashedTopup: (value: boolean) => void;

  setErrorMonthStatusSuccess: (value: string | null) => void;
  setErrorYearStatusSuccess: (value: string | null) => void;
  setErrorMonthStatusFailed: (value: string | null) => void;
  setErrorYearStatusFailed: (value: string | null) => void;
  setErrorMonthTopupMethod: (value: string | null) => void;
  setErrorYearTopupMethod: (value: string | null) => void;
  setErrorMonthTopupAmount: (value: string | null) => void;
  setErrorYearTopupAmount: (value: string | null) => void;

  setErrorGetTopups: (value: string | null) => void;
  setErrorGetTopup: (value: string | null) => void;
  setErrorGetActiveTopup: (value: string | null) => void;
  setErrorGetTrashedTopup: (value: string | null) => void;
  setErrorGetCardNumberTopup: (value: string | null) => void;

  setErrorCreateTopup: (value: string | null) => void;
  setErrorUpdateTopup: (value: string | null) => void;
  setErrorTrashedTopup: (value: string | null) => void;

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

  findMonthTopupMethod: (toast: any, year: number) => Promise<void>;
  findYearTopupMethod: (toast: any, year: number) => Promise<void>;
  findMonthTopupAmount: (toast: any, year: number) => Promise<void>;
  findYearTopupAmount: (toast: any, year: number) => Promise<void>;

  findMonthTopupMethodCard: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;
  findYearTopupMethodCard: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;
  findMonthTopupAmountCard: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;
  findYearTopupAmountCard: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;

  findAllTopups: (req: FindAllTopup) => Promise<void>;
  findByIdTopup: (req: FindByIdTopup) => Promise<void>;
  findByActiveTopup: (req: FindAllTopup) => Promise<void>;
  findByCardNumberTopup: (req: FindByCardNumberTopup) => Promise<void>;
  createTopup: (req: CreateTopup) => Promise<boolean>;
  updateTopup: (req: UpdateTopup) => Promise<boolean>;
  trashedTopup: (req: TrashedTopup) => Promise<boolean>;
}
