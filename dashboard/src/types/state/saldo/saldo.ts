import {
  Saldo,
  SaldoMonthBalance,
  SaldoMonthTotalBalance,
  SaldoYearBalance,
  SaldoYearTotalBalance,
} from "@/types/model/saldo";
import {
  CreateSaldo,
  FindAllSaldo,
  FindByIdSaldo,
  UpdateSaldo,
} from "@/types/domain/request";
import {
  FindByCardNumberSaldo,
  TrashedSaldo,
} from "@/types/domain/request/saldo";

export interface SaldoStore {
  saldos: Saldo[] | null;
  saldo: Saldo | null;

  monthTotalBalance: SaldoMonthTotalBalance[] | null;
  yearTotalBalance: SaldoYearTotalBalance[] | null;

  monthBalance: SaldoMonthBalance[] | null;
  yearBalance: SaldoYearBalance[] | null;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingMonthTotalBalance: boolean;
  loadingYearTotalBalance: boolean;

  loadingMonthBalance: boolean;
  loadingYearBalance: boolean;

  loadingGetSaldos: boolean;
  loadingGetSaldo: boolean;
  loadingGetActiveSaldo: boolean;
  loadingGetTrashedSaldo: boolean;
  loadingGetCardNumberSaldo: boolean;

  loadingCreateSaldo: boolean;
  loadingUpdateSaldo: boolean;
  loadingTrashedSaldo: boolean;

  errorMonthTotalBalance: string | null;
  errorYearTotalBalance: string | null;

  errorMonthBalance: string | null;
  errorYearBalance: string | null;

  errorGetSaldos: string | null;
  errorGetSaldo: string | null;
  errorGetActiveSaldo: string | null;
  errorGetTrashedSaldo: string | null;
  errorGetCardNumberSaldo: string | null;

  errorCreateSaldo: string | null;
  errorUpdateSaldo: string | null;
  errorTrashedSaldo: string | null;

  setLoadingMonthTotalBalance: (value: boolean) => void;
  setLoadingYearTotalBalance: (value: boolean) => void;
  setLoadingMonthBalance: (value: boolean) => void;
  setLoadingYearBalance: (value: boolean) => void;

  setLoadingGetSaldos: (value: boolean) => void;
  setLoadingGetSaldo: (value: boolean) => void;
  setLoadingGetActiveSaldo: (value: boolean) => void;
  setLoadingGetTrashedSaldo: (value: boolean) => void;
  setLoadingGetCardNumberSaldo: (value: boolean) => void;

  setLoadingCreateSaldo: (value: boolean) => void;
  setLoadingUpdateSaldo: (value: boolean) => void;
  setLoadingTrashedSaldo: (value: boolean) => void;

  setErrorMonthTotalBalance: (value: string | null) => void;
  setErrorYearTotalBalance: (value: string | null) => void;
  setErrorMonthBalance: (value: string | null) => void;
  setErrorYearBalance: (value: string | null) => void;

  setErrorGetSaldos: (value: string | null) => void;
  setErrorGetSaldo: (value: string | null) => void;
  setErrorGetActiveSaldo: (value: string | null) => void;
  setErrorGetTrashedSaldo: (value: string | null) => void;
  setErrorGetCardNumberSaldo: (value: string | null) => void;

  setErrorCreateSaldo: (value: string | null) => void;
  setErrorUpdateSaldo: (value: string | null) => void;
  setErrorTrashedSaldo: (value: string | null) => void;

  findMonthTotalBalance: (
    toast: any,
    year: number,
    month: number,
  ) => Promise<void>;
  findYearTotalBalance: (toast: any, year: number) => Promise<void>;
  findMonthBalance: (toast: any, year: number) => Promise<void>;
  findYearBalance: (toast: any, year: number) => Promise<void>;

  findAllSaldos: (req: FindAllSaldo) => Promise<void>;
  findByIdSaldo: (req: FindByIdSaldo) => Promise<void>;
  findByActiveSaldo: (req: FindAllSaldo) => Promise<void>;
  findByCardNumberSaldo: (req: FindByCardNumberSaldo) => Promise<void>;
  createSaldo: (req: CreateSaldo) => Promise<boolean>;
  updateSaldo: (req: UpdateSaldo) => Promise<boolean>;
  trashedSaldo: (req: TrashedSaldo) => Promise<boolean>;
}
