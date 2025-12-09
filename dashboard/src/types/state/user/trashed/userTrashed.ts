import { DeletePermanentUser } from "@/types/domain/request/user/trashed/delete";
import { FindAllUserTrashed } from "@/types/domain/request/user/trashed/list";
import { RestoreUserTrashed } from "@/types/domain/request/user/trashed/restore";

export interface UserTrashedStore {
  users: any;
  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingGetUsersTrashed: boolean | null;
  loadingRestoreUserTrashed: boolean | null;
  loadingDeletePermanentUserTrashed: boolean | null;
  loadingRestoreAllUserTrashed: boolean | null;
  loadingDeletePermanentAllUserTrashed: boolean | null;

  errorGetUsersTrashed: boolean | null;
  errorRestoreUserTrashed: boolean | null;
  errorDeletePermanentUserTrashed: boolean | null;
  errorRestoreAllUserTrashed: boolean | null;
  errorDeletePermanentAllUserTrashed: boolean | null;

  setLoadingGetUsersTrashed: (value: boolean) => void;
  setLoadingRestoreUserTrashed: (value: boolean) => void;
  setLoadingDeletePermanentUserTrashed: (value: boolean) => void;
  setLoadingRestoreAllUserTrashed: (value: boolean) => void;
  setLoadingDeletePermanentAllUserTrashed: (value: boolean) => void;

  setErrorGetUsersTrashed: (value: boolean) => void;
  setErrorRestoreUserTrashed: (value: boolean) => void;
  setErrorDeletePermanentUserTrashed: (value: boolean) => void;
  setErrorRestoreAllUserTrashed: (value: boolean) => void;
  setErrorDeletePermanentAllUserTrashed: (value: boolean) => void;

  findAllUsersTrashed: (req: FindAllUserTrashed) => Promise<void>;
  restoreUserTrashed: (req: RestoreUserTrashed) => Promise<boolean>;
  deletePermanentUser: (req: DeletePermanentUser) => Promise<boolean>;
  restoreUserAllTrashed: (toast: any) => Promise<boolean>;
  deletePermanentAllUser: (toast: any) => Promise<boolean>;
}
