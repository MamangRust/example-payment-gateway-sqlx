import { FindByUser } from "@/types/domain/request/card/user";
import {
  CreateCard,
  FindAllCard,
  FindByCardNumber,
  FindByIdCard,
  FindByIdTrashed,
  UpdateCard,
} from "../../domain/request";
import {
  Card,
  CardMonthBalance,
  CardYearlyBalance,
  CardMonthTopupAmount,
  CardMonthTransactionAmount,
  CardMonthTransferAmount,
  CardMonthWithdrawAmount,
  CardYearlyTopupAmount,
  CardYearlyTransactionAmount,
  CardYearlyTransferAmount,
  CardYearlyWithdrawAmount,
  DashboardCard,
  DashboardCardCardNumber,
} from "../../model/card";

export interface CardStore {
  cards: Card[] | null;
  card: Card | null;

  dashboard: DashboardCard | null;
  dashboardCardNumber: DashboardCardCardNumber | null;

  monthBalance: CardMonthBalance[] | null;
  yearBalance: CardYearlyBalance[] | null;

  monthTopupAmount: CardMonthTopupAmount[] | null;
  yearTopupAmount: CardYearlyTopupAmount[] | null;

  monthWithdrawAmount: CardMonthWithdrawAmount[] | null;
  yearWithdrawAmount: CardYearlyWithdrawAmount[] | null;

  monthTransactionAmount: CardMonthTransactionAmount[] | null;
  yearTransactionAmount: CardYearlyTransactionAmount[] | null;

  monthTransferSender: CardMonthTransferAmount[] | null;
  yearTransferSender: CardYearlyTransferAmount[] | null;

  monthTransferReceiver: CardMonthTransferAmount[] | null;
  yearTransferReceiver: CardYearlyTransferAmount[] | null;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingDashboard: boolean;
  loadingDashboardCardNumber: boolean;

  loadingMonthBalance: boolean;
  loadingYearBalance: boolean;

  loadingMonthTopupAmount: boolean;
  loadingYearTopupAmount: boolean;

  loadingMonthWithdrawAmount: boolean;
  loadingYearWithdrawAmount: boolean;

  loadingMonthTransaction: boolean;
  loadingYearTransaction: boolean;

  loadingMonthTransferSender: boolean;
  loadingYearTransferSender: boolean;

  loadingMonthTransferReceiver: boolean;
  loadingYearTransferReceiver: boolean;

  loadingGetCards: boolean;
  loadingGetCard: boolean;
  loadingGetCardByUser: boolean;
  loadingGetActiveCards: boolean;
  loadingGetCardByCardNumber: boolean;

  loadingCreateCard: boolean;
  loadingUpdateCard: boolean;
  loadingTrashedCard: boolean;

  errorDashboard: string | null;
  errorDashboardCardNumber: string | null;

  errorMonthBalance: string | null;
  errorYearBalance: string | null;

  errorMonthTopupAmount: string | null;
  errorYearTopupAmount: string | null;

  errorMonthWithdrawAmount: string | null;
  errorYearWithdrawAmount: string | null;

  errorMonthTransaction: string | null;
  errorYearTransaction: string | null;

  errorMonthTransferSender: string | null;
  errorYearTransferSender: string | null;

  errorMonthTransferReceiver: string | null;
  errorYearTransferReceiver: string | null;

  errorGetCards: string | null;
  errorGetCard: string | null;
  errorGetCardByUser: string | null;
  errorGetActiveCards: string | null;
  errorGetCardByCardNumber: string | null;

  errorCreateCard: string | null;
  errorUpdateCard: string | null;
  errorTrashedCard: string | null;

  setErrorDashboard: (value: string | null) => void;
  setErrorDashboardCardNumber: (value: string | null) => void;

  setErrorMonthBalance: (value: string | null) => void;
  setErrorYearBalance: (value: string | null) => void;

  setErrorMonthTopupAmount: (value: string | null) => void;
  setErrorYearTopupAmount: (value: string | null) => void;

  setErrorMonthWithdrawAmount: (value: string | null) => void;
  setErrorYearWithdrawAmount: (value: string | null) => void;

  setErrorMonthTransaction: (value: string | null) => void;
  setErrorYearTransaction: (value: string | null) => void;

  setErrorMonthTransferSender: (value: string | null) => void;
  setErrorYearTransferSender: (value: string | null) => void;

  setErrorMonthTransferReceiver: (value: string | null) => void;
  setErrorYearTransferReceiver: (value: string | null) => void;

  setErrorGetCards: (value: string | null) => void;
  setErrorGetCard: (value: string | null) => void;
  setErrorGetCardByUser: (value: string | null) => void;
  setErrorGetActiveCards: (value: string | null) => void;
  setErrorGetCardByCardNumber: (value: string | null) => void;

  setErrorCreateCard: (value: string | null) => void;
  setErrorUpdateCard: (value: string | null) => void;
  setErrorTrashedCard: (value: string | null) => void;

  setLoadingDashboard: (value: boolean) => void;
  setLoadingDashboardCardNumber: (value: boolean) => void;

  setLoadingMonthBalance: (value: boolean) => void;
  setLoadingYearBalance: (value: boolean) => void;
  setLoadingMonthTopupAmount: (value: boolean) => void;
  setLoadingYearTopupAmount: (value: boolean) => void;

  setLoadingMonthWithdrawAmount: (value: boolean) => void;
  setLoadingYearWithdrawAmount: (value: boolean) => void;
  setLoadingMonthTransaction: (value: boolean) => void;
  setLoadingYearTransaction: (value: boolean) => void;

  setLoadingMonthTransferSender: (value: boolean) => void;
  setLoadingYearTransferSender: (value: boolean) => void;

  setLoadingMonthTransferReceiver: (value: boolean) => void;
  setLoadingYearTransferReceiver: (value: boolean) => void;

  setLoadingGetCards: (value: boolean) => void;
  setLoadingGetCard: (value: boolean) => void;
  setLoadingGetCardByUser: (value: boolean) => void;
  setLoadingGetActiveCards: (value: boolean) => void;
  setLoadingGetCardByCardNumber: (value: boolean) => void;

  setLoadingCreateCard: (value: boolean) => void;
  setLoadingUpdateCard: (value: boolean) => void;
  setLoadingTrashedCard: (value: boolean) => void;

  dashboardCard: (toast: any) => Promise<void>;
  dashboardCardCardNumber: (toast: any, card_number: string) => Promise<void>;
  findMonthBalance: (toast: any, year: number) => Promise<void>;
  findYearBalance: (toast: any, year: number) => Promise<void>;
  findMonthTopupAmount: (toast: any, year: number) => Promise<void>;
  findYearTopupAmount: (toast: any, year: number) => Promise<void>;
  findMonthWithdrawAmount: (toast: any, year: number) => Promise<void>;
  findYearWithdrawAmount: (toast: any, year: number) => Promise<void>;
  findMonthlyTransactionAmount: (toast: any, year: number) => Promise<void>;
  findYearlyTransactionAmount: (toast: any, year: number) => Promise<void>;
  findMonthlyTransferSenderAmount: (toast: any, year: number) => Promise<void>;
  findYearlyTransferSenderAmount: (toast: any, year: number) => Promise<void>;
  findMonthlyTransferReceiverAmount: (
    toast: any,
    year: number,
  ) => Promise<void>;
  findYearlyTransferReceiverAmount: (toast: any, year: number) => Promise<void>;

  findMonthlyBalanceByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;
  findYearlyBalanceByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;
  findMonthlyTopupAmountByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;
  findYearlyTopupAmountByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;
  findMonthlyWithdrawAmountByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;
  findYearlyWithdrawAmountByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;

  findMonthlyTransactionAmountByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;
  findYearlyTransactionAmountByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;

  findMonthlyTransferSenderAmountByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;
  findYearlyTransferSenderAmountByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;

  findMonthlyTransferReceiverAmountByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;
  findYearlyTransferReceiverAmountByCard: (
    toast: any,
    year: number,
    card_number: String,
  ) => Promise<void>;

  findAllCards: (req: FindAllCard) => Promise<void>;
  findByIdCard: (req: FindByIdCard) => Promise<void>;

  findByUser: (req: FindByUser) => Promise<void>;
  findByCardNumber: (req: FindByCardNumber) => Promise<void>;

  findByActiveCard: (req: FindAllCard) => Promise<void>;

  createCard: (req: CreateCard) => Promise<boolean>;
  updateCard: (req: UpdateCard) => Promise<boolean>;
  trashedCard: (req: FindByIdTrashed) => Promise<boolean>;
}
