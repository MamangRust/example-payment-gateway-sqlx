import {
  CreateTransaction,
  FindAllTransaction,
  FindyByCardNumberTransaction,
  FindyByIdTransaction,
  FindyByMerchantTransaction,
  TrashedTransaction,
  UpdateTransaction,
} from "../../domain/request";
import {
  Transaction,
  TransactionMonthAmount,
  TransactionMonthMethod,
  TransactionMonthStatusFailed,
  TransactionMonthStatusSuccess,
  TransactionYearlyAmount,
  TransactionYearMethod,
  TransactionYearStatusFailed,
  TransactionYearStatusSuccess,
} from "../../model/transaction";

export interface TransactionStore {
  transactions: Transaction[] | null;
  transaction: Transaction | null;

  monthStatusSuccess: TransactionMonthStatusSuccess[] | null;
  yearStatusSuccess: TransactionYearStatusSuccess[] | null;

  monthStatusFailed: TransactionMonthStatusFailed[] | null;
  yearStatusFailed: TransactionYearStatusFailed[] | null;

  monthTransactionMethod: TransactionMonthMethod[] | null;
  yearTransactionMethod: TransactionYearMethod[] | null;

  monthTransactionAmount: TransactionMonthAmount[] | null;
  yearTransactionAmount: TransactionYearlyAmount[] | null;

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
  loadingMonthTransactionMethod: boolean;
  loadingYearTransactionMethod: boolean;
  loadingMonthTransactionAmount: boolean;
  loadingYearTransactionAmount: boolean;

  loadingGetTransactions: boolean;
  loadingGetTransaction: boolean;
  loadingGetCardNumberTransaction: boolean;
  loadingGetMerchantTransaction: boolean;
  loadingGetActiveTransaction: boolean;
  loadingGetTrashedTransaction: boolean;

  loadingCreateTransaction: boolean;
  loadingUpdateTransaction: boolean;
  loadingTrashedTransaction: boolean;

  errorMonthStatusSuccess: string | null;
  errorYearStatusSuccess: string | null;
  errorMonthStatusFailed: string | null;
  errorYearStatusFailed: string | null;
  errorMonthTransactionMethod: string | null;
  errorYearTransactionMethod: string | null;
  errorMonthTransactionAmount: string | null;
  errorYearTransactionAmount: string | null;

  errorGetTransactions: string | null;
  errorGetTransaction: string | null;
  errorGetCardNumberTransaction: string | null;
  errorGetMerchantTransaction: string | null;
  errorGetActiveTransaction: string | null;
  errorGetTrashedTransaction: string | null;

  errorCreateTransaction: string | null;
  errorUpdateTransaction: string | null;
  errorTrashedTransaction: string | null;

  setLoadingMonthStatusSuccess: (value: boolean) => void;
  setLoadingYearStatusSuccess: (value: boolean) => void;
  setLoadingMonthStatusFailed: (value: boolean) => void;
  setLoadingYearStatusFailed: (value: boolean) => void;
  setLoadingMonthTransactionMethod: (value: boolean) => void;
  setLoadingYearTransactionMethod: (value: boolean) => void;

  setLoadingMonthTransactionAmount: (value: boolean) => void;
  setLoadingYearTransactionAmount: (value: boolean) => void;

  setLoadingGetTransactions: (value: boolean) => void;
  setLoadingGetTransaction: (value: boolean) => void;
  setLoadingGetCardNumberTransaction: (value: boolean) => void;
  setLoadingGetMerchantTransaction: (value: boolean) => void;
  setLoadingGetActiveTransaction: (value: boolean) => void;
  setLoadingGetTrashedTransaction: (value: boolean) => void;

  setLoadingCreateTransaction: (value: boolean) => void;
  setLoadingUpdateTransaction: (value: boolean) => void;
  setLoadingTrashedTransaction: (value: boolean) => void;

  setErrorMonthStatusSuccess: (value: string | null) => void;
  setErrorYearStatusSuccess: (value: string | null) => void;
  setErrorMonthStatusFailed: (value: string | null) => void;
  setErrorYearStatusFailed: (value: string | null) => void;
  setErrorMonthTransactionMethod: (value: string | null) => void;
  setErrorYearTransactionMethod: (value: string | null) => void;
  setErrorMonthTransactionAmount: (value: string | null) => void;
  setErrorYearTransactionAmount: (value: string | null) => void;

  setErrorGetTransactions: (value: string | null) => void;
  setErrorGetTransaction: (value: string | null) => void;
  setErrorGetCardNumberTransaction: (value: string | null) => void;
  setErrorGetMerchantTransaction: (value: string | null) => void;
  setErrorGetActiveTransaction: (value: string | null) => void;
  setErrorGetTrashedTransaction: (value: string | null) => void;

  setErrorCreateTransaction: (value: string | null) => void;
  setErrorUpdateTransaction: (value: string | null) => void;
  setErrorTrashedTransaction: (value: string | null) => void;

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

  findMonthTransactionMethod: (toast: any, year: number) => Promise<void>;
  findYearTransactionMethod: (toast: any, year: number) => Promise<void>;

  findMonthTransactionAmount: (toast: any, year: number) => Promise<void>;
  findYearTransactionAmount: (toast: any, year: number) => Promise<void>;

  findMonthTransactionMethodCard: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;
  findYearTransactionMethodCard: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;

  findMonthTransactionAmountCard: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;
  findYearTransactionAmountCard: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;

  findAllTransactions: (req: FindAllTransaction) => Promise<void>;
  findByIdTransaction: (req: FindyByIdTransaction) => Promise<void>;
  findByCardNumberTransaction: (
    req: FindyByCardNumberTransaction,
  ) => Promise<void>;
  findByMerchantTransaction: (req: FindyByMerchantTransaction) => Promise<void>;
  findByActiveTransaction: (req: FindAllTransaction) => Promise<void>;
  createTransaction: (req: CreateTransaction) => Promise<boolean>;
  updateTransaction: (req: UpdateTransaction) => Promise<boolean>;
  trashedTransaction: (req: TrashedTransaction) => Promise<boolean>;
}
