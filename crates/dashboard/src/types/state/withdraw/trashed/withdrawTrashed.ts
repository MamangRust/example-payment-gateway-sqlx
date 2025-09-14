import {
  DeletePermanentWithdraw,
  FindAllWithdrawTrashed,
  RestoreWithdrawTrashed,
} from "@/types/domain/request/withdraw/trashed";

export interface WithdrawTrashedStore {
  withdraws: any;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingGetWithdrawsTrashed: boolean | null;
  loadingRestoreWithdrawTrashed: boolean | null;
  loadingDeletePermanentWithdrawTrashed: boolean | null;
  loadingRestoreAllWithdrawTrashed: boolean | null;
  loadingDeletePermanentAllWithdrawTrashed: boolean | null;

  errorGetWithdrawsTrashed: boolean | null;
  errorRestoreWithdrawTrashed: boolean | null;
  errorDeletePermanentWithdrawTrashed: boolean | null;
  errorRestoreAllWithdrawTrashed: boolean | null;
  errorDeletePermanentAllWithdrawTrashed: boolean | null;

  setLoadingGetWithdrawsTrashed: (value: boolean) => void;
  setLoadingRestoreWithdrawTrashed: (value: boolean) => void;
  setLoadingDeletePermanentWithdrawTrashed: (value: boolean) => void;
  setLoadingRestoreAllWithdrawTrashed: (value: boolean) => void;
  setLoadingDeletePermanentAllWithdrawTrashed: (value: boolean) => void;

  setErrorGetWithdrawsTrashed: (value: boolean) => void;
  setErrorRestoreWithdrawTrashed: (value: boolean) => void;
  setErrorDeletePermanentWithdrawTrashed: (value: boolean) => void;
  setErrorRestoreAllWithdrawTrashed: (value: boolean) => void;
  setErrorDeletePermanentAllWithdrawTrashed: (value: boolean) => void;

  findAllWithdrawsTrashed: (req: FindAllWithdrawTrashed) => Promise<void>;
  restoreWithdrawTrashed: (req: RestoreWithdrawTrashed) => Promise<boolean>;
  deletePermanentWithdraw: (req: DeletePermanentWithdraw) => Promise<boolean>;
  restoreWithdrawAllTrashed: (toast: any) => Promise<boolean>;
  deletePermanentAllWithdraw: (toast: any) => Promise<boolean>;
}
