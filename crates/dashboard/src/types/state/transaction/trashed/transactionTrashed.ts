import {
  DeletePermanentTransaction,
  FindAllTransactionTrashed,
  RestoreTransactionTrashed,
} from "@/types/domain/request/transaction/trashed";

export interface TransactionTrashedStore {
  transactions: any;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingGetTransactionsTrashed: boolean | null;
  loadingRestoreTransactionTrashed: boolean | null;
  loadingDeletePermanentTransactionTrashed: boolean | null;
  loadingRestoreAllTransactionTrashed: boolean | null;
  loadingDeletePermanentAllTransactionTrashed: boolean | null;

  errorGetTransactionsTrashed: boolean | null;
  errorRestoreTransactionTrashed: boolean | null;
  errorDeletePermanentTransactionTrashed: boolean | null;
  errorRestoreAllTransactionTrashed: boolean | null;
  errorDeletePermanentAllTransactionTrashed: boolean | null;

  setLoadingGetTransactionsTrashed: (value: boolean) => void;
  setLoadingRestoreTransactionTrashed: (value: boolean) => void;
  setLoadingDeletePermanentTransactionTrashed: (value: boolean) => void;
  setLoadingRestoreAllTransactionTrashed: (value: boolean) => void;
  setLoadingDeletePermanentAllTransactionTrashed: (value: boolean) => void;

  setErrorGetTransactionsTrashed: (value: boolean) => void;
  setErrorRestoreTransactionTrashed: (value: boolean) => void;
  setErrorDeletePermanentTransactionTrashed: (value: boolean) => void;
  setErrorRestoreAllTransactionTrashed: (value: boolean) => void;
  setErrorDeletePermanentAllTransactionTrashed: (value: boolean) => void;

  findAllTransactionsTrashed: (req: FindAllTransactionTrashed) => Promise<void>;
  restoreTransactionTrashed: (
    req: RestoreTransactionTrashed,
  ) => Promise<boolean>;
  deletePermanentTransaction: (
    req: DeletePermanentTransaction,
  ) => Promise<boolean>;
  restoreTransactionAllTrashed: (toast: any) => Promise<boolean>;
  deletePermanentAllTransaction: (toast: any) => Promise<boolean>;
}
