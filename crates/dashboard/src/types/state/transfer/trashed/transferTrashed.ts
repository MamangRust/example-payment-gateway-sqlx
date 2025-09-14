import {
  DeletePermanentTransfer,
  FindAllTransferTrashed,
  RestoreTransferTrashed,
} from "@/types/domain/request/transfer/trashed";

export interface TransferTrashedStore {
  transfers: any;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingGetTransfersTrashed: boolean | null;
  loadingRestoreTransferTrashed: boolean | null;
  loadingDeletePermanentTransferTrashed: boolean | null;
  loadingRestoreAllTransferTrashed: boolean | null;
  loadingDeletePermanentAllTransferTrashed: boolean | null;

  errorGetTransfersTrashed: boolean | null;
  errorRestoreTransferTrashed: boolean | null;
  errorDeletePermanentTransferTrashed: boolean | null;
  errorRestoreAllTransferTrashed: boolean | null;
  errorDeletePermanentAllTransferTrashed: boolean | null;

  setLoadingGetTransfersTrashed: (value: boolean) => void;
  setLoadingRestoreTransferTrashed: (value: boolean) => void;
  setLoadingDeletePermanentTransferTrashed: (value: boolean) => void;
  setLoadingRestoreAllTransferTrashed: (value: boolean) => void;
  setLoadingDeletePermanentAllTransferTrashed: (value: boolean) => void;

  setErrorGetTransfersTrashed: (value: boolean) => void;
  setErrorRestoreTransferTrashed: (value: boolean) => void;
  setErrorDeletePermanentTransferTrashed: (value: boolean) => void;
  setErrorRestoreAllTransferTrashed: (value: boolean) => void;
  setErrorDeletePermanentAllTransferTrashed: (value: boolean) => void;

  findAllTransfersTrashed: (req: FindAllTransferTrashed) => Promise<void>;
  restoreTransferTrashed: (req: RestoreTransferTrashed) => Promise<boolean>;
  deletePermanentTransfer: (req: DeletePermanentTransfer) => Promise<boolean>;
  restoreTransferAllTrashed: (toast: any) => Promise<boolean>;
  deletePermanentAllTransfer: (toast: any) => Promise<boolean>;
}
