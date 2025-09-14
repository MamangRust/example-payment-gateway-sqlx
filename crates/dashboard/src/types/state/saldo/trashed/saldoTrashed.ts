import {
  DeletePermanentSaldo,
  FindAllSaldoTrashed,
  RestoreSaldoTrashed,
} from "@/types/domain/request";

export interface SaldoTrashedStore {
  saldos: any;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingGetSaldosTrashed: boolean | null;
  loadingRestoreSaldoTrashed: boolean | null;
  loadingDeletePermanentSaldoTrashed: boolean | null;
  loadingRestoreAllSaldoTrashed: boolean | null;
  loadingDeletePermanentAllSaldoTrashed: boolean | null;

  errorGetSaldosTrashed: boolean | null;
  errorRestoreSaldoTrashed: boolean | null;
  errorDeletePermanentSaldoTrashed: boolean | null;
  errorRestoreAllSaldoTrashed: boolean | null;
  errorDeletePermanentAllSaldoTrashed: boolean | null;

  setLoadingGetSaldosTrashed: (value: boolean) => void;
  setLoadingRestoreSaldoTrashed: (value: boolean) => void;
  setLoadingDeletePermanentSaldoTrashed: (value: boolean) => void;
  setLoadingRestoreAllSaldoTrashed: (value: boolean) => void;
  setLoadingDeletePermanentAllSaldoTrashed: (value: boolean) => void;

  setErrorGetSaldosTrashed: (value: boolean) => void;
  setErrorRestoreSaldoTrashed: (value: boolean) => void;
  setErrorDeletePermanentSaldoTrashed: (value: boolean) => void;
  setErrorRestoreAllSaldoTrashed: (value: boolean) => void;
  setErrorDeletePermanentAllSaldoTrashed: (value: boolean) => void;

  findAllSaldosTrashed: (req: FindAllSaldoTrashed) => Promise<void>;

  restoreSaldoTrashed: (req: RestoreSaldoTrashed) => Promise<boolean>;
  deletePermanentSaldo: (req: DeletePermanentSaldo) => Promise<boolean>;
  restoreSaldoAllTrashed: (toast: any) => Promise<boolean>;
  deletePermanentAllSaldo: (toast: any) => Promise<boolean>;
}
