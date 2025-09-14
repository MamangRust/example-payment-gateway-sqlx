import {
  CreateMerchant,
  FindAllMerchant,
  FindAllMerchantTransaction,
  FindAllTransactionByApiKey,
  findByApiKeyMerchant,
  FindByIdMerchant,
  FindMerchantUser,
  FindTrashedMerchant,
  UpdateMerchant,
} from "../../domain/request";
import {
  Merchant,
  MerchantMonthlyAmount,
  MerchantMonthlyPaymentMethod,
  MerchantMonthlyTotalAmount,
  MerchantTransaction,
  MerchantYearlyAmount,
  MerchantYearlyPaymentMethod,
  MerchantYearlyTotalAmount,
} from "../../model/merchant";

export interface MerchantStore {
  merchants: Merchant[] | null;
  merchant: Merchant | null;

  transactions: MerchantTransaction[] | null;

  monthPaymentMethod: MerchantMonthlyPaymentMethod[] | null;
  yearPaymentMethod: MerchantYearlyPaymentMethod[] | null;

  monthAmount: MerchantMonthlyAmount[] | null;
  yearAmount: MerchantYearlyAmount[] | null;

  monthTotalAmount: MerchantMonthlyTotalAmount[] | null;
  yearTotalAmount: MerchantYearlyTotalAmount[] | null;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  paginationTransaction: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingMonthPaymentMethod: boolean;
  loadingYearPaymentMethod: boolean;

  loadingMonthAmount: boolean;
  loadingYearAmount: boolean;

  loadingMonthTotalAmount: boolean;
  loadingYearTotalAmount: boolean;

  errorMonthPaymentMethod: string | null;
  errorYearPaymentMethod: string | null;

  errorMonthAmount: string | null;
  errorYearAmount: string | null;
  errorMonthTotalAmount: string | null;
  errorYearTotalAmount: string | null;

  loadingGetMerchants: boolean;
  loadingGetTransactions: boolean;
  loadingGetMerchant: boolean;
  loadingGetApiKey: boolean;
  loadingGetMerchantUser: boolean;
  loadingGetActiveMerchant: boolean;
  loadingGetTrashedMerchant: boolean;

  loadingCreateMerchant: boolean;
  loadingUpdateMerchant: boolean;
  loadingTrashedMerchant: boolean;
  loadingRestoreMerchant: boolean;
  loadingDeletePermanentMerchant: boolean;

  errorGetMerchants: string | null;
  errorGetTransactions: string | null;
  errorGetMerchant: string | null;
  errorGetApiKey: string | null;
  errorGetMerchantUser: string | null;
  errorGetActiveMerchant: string | null;
  errorGetTrashedMerchant: string | null;

  errorCreateMerchant: string | null;
  errorUpdateMerchant: string | null;
  errorTrashedMerchant: string | null;

  setLoadingMonthPaymentMethod: (value: boolean) => void;
  setLoadingYearPaymentMethod: (value: boolean) => void;
  setLoadingMonthAmount: (value: boolean) => void;
  setLoadingYearAmount: (value: boolean) => void;
  setLoadingMonthTotalAmount: (value: boolean) => void;
  setLoadingYearTotalAmount: (value: boolean) => void;

  setLoadingGetMerchants: (value: boolean) => void;
  setLoadingGetTransactions: (value: boolean) => void;
  setLoadingGetMerchant: (value: boolean) => void;
  setLoadingGetApiKey: (value: boolean) => void;
  setLoadingGetMerchantUser: (value: boolean) => void;
  setLoadingGetActiveMerchant: (value: boolean) => void;
  setLoadingGetTrashedMerchant: (value: boolean) => void;
  setLoadingCreateMerchant: (value: boolean) => void;
  setLoadingUpdateMerchant: (value: boolean) => void;
  setLoadingTrashedMerchant: (value: boolean) => void;

  setErrorMonthPaymentMethod: (value: string | null) => void;
  setErrorYearPaymentMethod: (value: string | null) => void;
  setErrorMonthAmount: (value: string | null) => void;
  setErrorYearAmount: (value: string | null) => void;

  setErrorMonthTotalAmount: (value: string | null) => void;
  setErrorYearTotalAmount: (value: string | null) => void;

  setErrorGetMerchants: (value: string | null) => void;
  setErrorGetTransactions: (value: string | null) => void;
  setErrorGetMerchant: (value: string | null) => void;
  setErrorGetApiKey: (value: string | null) => void;
  setErrorGetMerchantUser: (value: string | null) => void;
  setErrorGetActiveMerchant: (value: string | null) => void;
  setErrorGetTrashedMerchant: (value: string | null) => void;
  setErrorCreateMerchant: (value: string | null) => void;
  setErrorUpdateMerchant: (value: string | null) => void;
  setErrorTrashedMerchant: (value: string | null) => void;

  findMonthPaymentMethod: (toast: any, year: number) => Promise<void>;
  findYearPaymentMethod: (toast: any, year: number) => Promise<void>;
  findMonthAmount: (toast: any, year: number) => Promise<void>;
  findYearAmount: (toast: any, year: number) => Promise<void>;
  findMonthTotalAmount: (
    toast: any,
    year: number,
    month: number,
  ) => Promise<void>;
  findYearTotalAmount: (toast: any, year: number) => Promise<void>;

  findMonthPaymentMethodByMerchant: (
    toast: any,
    year: number,
    merchant_id: number,
  ) => Promise<void>;

  findYearPaymentMethodByMerchant: (
    toast: any,
    year: number,
    merchant_id: number,
  ) => Promise<void>;
  findMonthAmountByMerchant: (
    toast: any,
    year: number,
    merchant_id: number,
  ) => Promise<void>;

  findYearAmountByMerchant: (
    toast: any,
    year: number,
    merchant_id: number,
  ) => Promise<void>;

  findMonthTotalAmountByMerchant: (
    toast: any,
    year: number,
    month: number,
    merchant_id: number,
  ) => Promise<void>;

  findYearTotalAmountByMerchant: (
    toast: any,
    year: number,

    merchant_id: number,
  ) => Promise<void>;

  findMonthPaymentMethodByApiKey: (
    toast: any,
    year: number,
    api_key: string,
  ) => Promise<void>;

  findYearPaymentMethodByApiKey: (
    toast: any,
    year: number,
    api_key: string,
  ) => Promise<void>;
  findMonthAmountByApiKey: (
    toast: any,
    year: number,
    api_key: string,
  ) => Promise<void>;

  findYearAmountByApiKey: (
    toast: any,
    year: number,
    api_key: string,
  ) => Promise<void>;

  findMonthTotalAmountByApiKey: (
    toast: any,
    year: number,
    month: number,
    api_key: string,
  ) => Promise<void>;

  findYearTotalAmountByApiKey: (
    toast: any,
    year: number,
    api_key: string,
  ) => Promise<void>;

  findAllMerchants: (req: FindAllMerchant) => Promise<void>;
  findAllTransaction: (req: FindAllMerchant) => Promise<void>;
  findAllTransactionByMerchant: (
    req: FindAllMerchantTransaction,
  ) => Promise<void>;
  findAllTransactionByApiKey: (
    req: FindAllTransactionByApiKey,
  ) => Promise<void>;

  findById: (req: FindByIdMerchant) => Promise<void>;
  findByApiKey: (req: findByApiKeyMerchant) => Promise<void>;
  findByMerchantUser: (req: FindMerchantUser) => Promise<void>;

  createMerchant: (req: CreateMerchant) => Promise<boolean>;
  updateMerchant: (req: UpdateMerchant) => Promise<boolean>;
  trashedMerchant: (req: FindTrashedMerchant) => Promise<boolean>;
}
