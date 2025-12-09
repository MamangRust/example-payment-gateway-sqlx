import {
  DeletePermanentTopup,
  FindAllTopupTrashed,
  RestoreTopupTrashed,
} from "@/types/domain/request/topup";

export interface TopupTrashedStore {
  topups: any;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingGetTopupsTrashed: boolean | null;
  loadingRestoreTopupTrashed: boolean | null;
  loadingDeletePermanentTopupTrashed: boolean | null;
  loadingRestoreAllTopupTrashed: boolean | null;
  loadingDeletePermanentAllTopupTrashed: boolean | null;

  errorGetTopupsTrashed: boolean | null;
  errorRestoreTopupTrashed: boolean | null;
  errorDeletePermanentTopupTrashed: boolean | null;
  errorRestoreAllTopupTrashed: boolean | null;
  errorDeletePermanentAllTopupTrashed: boolean | null;

  setLoadingGetTopupsTrashed: (value: boolean) => void;
  setLoadingRestoreTopupTrashed: (value: boolean) => void;
  setLoadingDeletePermanentTopupTrashed: (value: boolean) => void;
  setLoadingRestoreAllTopupTrashed: (value: boolean) => void;
  setLoadingDeletePermanentAllTopupTrashed: (value: boolean) => void;

  setErrorGetTopupsTrashed: (value: boolean) => void;
  setErrorRestoreTopupTrashed: (value: boolean) => void;
  setErrorDeletePermanentTopupTrashed: (value: boolean) => void;
  setErrorRestoreAllTopupTrashed: (value: boolean) => void;
  setErrorDeletePermanentAllTopupTrashed: (value: boolean) => void;

  findAllTopupsTrashed: (req: FindAllTopupTrashed) => Promise<void>;

  restoreTopupTrashed: (req: RestoreTopupTrashed) => Promise<boolean>;
  deletePermanentTopup: (req: DeletePermanentTopup) => Promise<boolean>;
  restoreTopupAllTrashed: (toast: any) => Promise<boolean>;
  deletePermanentAllTopup: (toast: any) => Promise<boolean>;
}
