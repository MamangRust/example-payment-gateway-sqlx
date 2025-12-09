import { DeletePermanentRole } from "@/types/domain/request/role/trashed/delete";
import { FindAllRoleTrashed } from "@/types/domain/request/role/trashed/list";
import { RestoreRoleTrashed } from "@/types/domain/request/role/trashed/restore";

export interface RoleTrashedStore {
  Roles: any;
  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingGetRolesTrashed: boolean | null;
  loadingRestoreRoleTrashed: boolean | null;
  loadingDeletePermanentRoleTrashed: boolean | null;
  loadingRestoreAllRoleTrashed: boolean | null;
  loadingDeletePermanentAllRoleTrashed: boolean | null;

  errorGetRolesTrashed: boolean | null;
  errorRestoreRoleTrashed: boolean | null;
  errorDeletePermanentRoleTrashed: boolean | null;
  errorRestoreAllRoleTrashed: boolean | null;
  errorDeletePermanentAllRoleTrashed: boolean | null;

  setLoadingGetRolesTrashed: (value: boolean) => void;
  setLoadingRestoreRoleTrashed: (value: boolean) => void;
  setLoadingDeletePermanentRoleTrashed: (value: boolean) => void;
  setLoadingRestoreAllRoleTrashed: (value: boolean) => void;
  setLoadingDeletePermanentAllRoleTrashed: (value: boolean) => void;

  setErrorGetRolesTrashed: (value: boolean) => void;
  setErrorRestoreRoleTrashed: (value: boolean) => void;
  setErrorDeletePermanentRoleTrashed: (value: boolean) => void;
  setErrorRestoreAllRoleTrashed: (value: boolean) => void;
  setErrorDeletePermanentAllRoleTrashed: (value: boolean) => void;

  findAllRolesTrashed: (req: FindAllRoleTrashed) => Promise<void>;
  restoreRoleTrashed: (req: RestoreRoleTrashed) => Promise<boolean>;
  deletePermanentRole: (req: DeletePermanentRole) => Promise<boolean>;
  restoreRoleAllTrashed: (toast: any) => Promise<boolean>;
  deletePermanentAllRole: (toast: any) => Promise<boolean>;
}
