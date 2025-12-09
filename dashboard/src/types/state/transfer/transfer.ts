import {
  FindAllTransfer,
  FindByIdTransfer,
  TransferFrom,
  TransferTo,
  CreateTransfer,
  UpdateTransfer,
  TrashedTransfer,
} from "@/types/domain/request";
import {
  Transfer,
  TransferMonthAmount,
  TransferMonthStatusFailed,
  TransferMonthStatusSuccess,
  TransferYearAmount,
  TransferYearStatusFailed,
  TransferYearStatusSuccess,
} from "../../model/transfer";

export interface TransferStore {
  transfers: Transfer[] | null;
  transfer: Transfer | null;

  monthStatusSuccess: TransferMonthStatusSuccess[] | null;
  yearStatusSuccess: TransferYearStatusSuccess[] | null;

  monthStatusFailed: TransferMonthStatusFailed[] | null;
  yearStatusFailed: TransferYearStatusFailed[] | null;

  monthTransferAmount: TransferMonthAmount[] | null;
  yearTransferAmount: TransferYearAmount[] | null;

  monthTransferAmountSender: TransferMonthAmount[] | null;
  yearTransferAmountSender: TransferYearAmount[] | null;

  monthTransferAmountReceiver: TransferMonthAmount[] | null;
  yearTransferAmountReceiver: TransferYearAmount[] | null;

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
  loadingMonthTransferAmount: boolean;
  loadingYearTransferAmount: boolean;

  loadingMonthTransferAmountSender: boolean;
  loadingYearTransferAmountSender: boolean;

  loadingMonthTransferAmountReceiver: boolean;
  loadingYearTransferAmountReceiver: boolean;

  loadingGetTransfers: boolean;
  loadingGetTransfer: boolean;
  loadingGetTransferFrom: boolean;
  loadingGetTransferTo: boolean;
  loadingGetActiveTransfer: boolean;
  loadingGetTrashedTransfer: boolean;

  loadingCreateTransfer: boolean;
  loadingUpdateTransfer: boolean;
  loadingTrashedTransfer: boolean;

  errorMonthStatusSuccess: string | null;
  errorYearStatusSuccess: string | null;
  errorMonthStatusFailed: string | null;
  errorYearStatusFailed: string | null;
  errorMonthTransferAmount: string | null;
  errorYearTransferAmount: string | null;

  errorMonthTransferAmountSender: string | null;
  errorYearTransferAmountSender: string | null;

  errorMonthTransferAmountReceiver: string | null;
  errorYearTransferAmountReceiver: string | null;

  errorGetTransfers: string | null;
  errorGetTransfer: string | null;
  errorGetTransferFrom: string | null;
  errorGetTransferTo: string | null;
  errorGetActiveTransfer: string | null;
  errorGetTrashedTransfer: string | null;

  errorCreateTransfer: string | null;
  errorUpdateTransfer: string | null;
  errorTrashedTransfer: string | null;

  setLoadingMonthStatusSuccess: (value: boolean) => void;
  setLoadingYearStatusSuccess: (value: boolean) => void;
  setLoadingMonthStatusFailed: (value: boolean) => void;
  setLoadingYearStatusFailed: (value: boolean) => void;
  setLoadingMonthTransferAmount: (value: boolean) => void;
  setLoadingYearTransferAmount: (value: boolean) => void;

  setLoadingMonthTransferAmountSender: (value: boolean) => void;
  setLoadingYearTransferAmountSender: (value: boolean) => void;

  setLoadingMonthTransferAmountReceiver: (value: boolean) => void;
  setLoadingYearTransferAmountReceiver: (value: boolean) => void;

  setLoadingGetTransfers: (value: boolean) => void;
  setLoadingGetTransfer: (value: boolean) => void;
  setLoadingGetTransferFrom: (value: boolean) => void;
  setLoadingGetTransferTo: (value: boolean) => void;
  setLoadingGetActiveTransfer: (value: boolean) => void;
  setLoadingGetTrashedTransfer: (value: boolean) => void;

  setLoadingCreateTransfer: (value: boolean) => void;
  setLoadingUpdateTransfer: (value: boolean) => void;
  setLoadingTrashedTransfer: (value: boolean) => void;

  setErrorMonthStatusSuccess: (value: string | null) => void;
  setErrorYearStatusSuccess: (value: string | null) => void;
  setErrorMonthStatusFailed: (value: string | null) => void;
  setErrorYearStatusFailed: (value: string | null) => void;
  setErrorMonthTransferAmount: (value: string | null) => void;
  setErrorYearTransferAmount: (value: string | null) => void;

  setErrorMonthTransferAmountSender: (value: string | null) => void;
  setErrorYearTransferAmountSender: (value: string | null) => void;

  setErrorMonthTransferAmountReceiver: (value: string | null) => void;
  setErrorYearTransferAmountReceiver: (value: string | null) => void;

  setErrorGetTransfers: (value: string | null) => void;
  setErrorGetTransfer: (value: string | null) => void;
  setErrorGetTransferFrom: (value: string | null) => void;
  setErrorGetTransferTo: (value: string | null) => void;
  setErrorGetActiveTransfer: (value: string | null) => void;
  setErrorGetTrashedTransfer: (value: string | null) => void;

  setErrorCreateTransfer: (value: string | null) => void;
  setErrorUpdateTransfer: (value: string | null) => void;
  setErrorTrashedTransfer: (value: string | null) => void;

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

  findMonthTransferAmount: (toast: any, year: number) => Promise<void>;
  findYearTransferAmount: (toast: any, year: number) => Promise<void>;

  findMonthTransferAmountBySender: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;
  findYearTransferAmountBySender: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;
  findMonthTransferAmountByReceiver: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;
  findYearTransferAmountByReceiver: (
    toast: any,
    year: number,
    card_number: string,
  ) => Promise<void>;

  findAllTransfers: (req: FindAllTransfer) => Promise<void>;
  findByIdTransfer: (req: FindByIdTransfer) => Promise<void>;
  findByTransferFrom: (req: TransferFrom) => Promise<void>;
  findByTransferTo: (req: TransferTo) => Promise<void>;
  findByActiveTransfer: (req: FindAllTransfer) => Promise<void>;
  createTransfer: (req: CreateTransfer) => Promise<boolean>;
  updateTransfer: (req: UpdateTransfer) => Promise<boolean>;
  trashedTransfer: (req: TrashedTransfer) => Promise<boolean>;
}
