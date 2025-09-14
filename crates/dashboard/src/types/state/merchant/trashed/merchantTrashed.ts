import { DeletePermanentMerchant } from "@/types/domain/request/merchant/trashed/delete";
import { FindAllMerchantTrashed } from "@/types/domain/request/merchant/trashed/list";
import { RestoreMerchantTrashed } from "@/types/domain/request/merchant/trashed/restore";

export interface MerchantTrashedStore {
  merchants: any;

  pagination: {
    currentPage: number;
    page_size: number;
    totalItems: number;
    totalPages: number;
  };

  loadingGetMerchantsTrashed: boolean | null;
  loadingRestoreMerchantTrashed: boolean | null;
  loadingDeletePermanentMerchantTrashed: boolean | null;
  loadingRestoreAllMerchantTrashed: boolean | null;
  loadingDeletePermanentAllMerchantTrashed: boolean | null;

  errorGetMerchantsTrashed: boolean | null;
  errorRestoreMerchantTrashed: boolean | null;
  errorDeletePermanentMerchantTrashed: boolean | null;
  errorRestoreAllMerchantTrashed: boolean | null;
  errorDeletePermanentAllMerchantTrashed: boolean | null;

  setLoadingGetMerchantsTrashed: (value: boolean) => void;
  setLoadingRestoreMerchantTrashed: (value: boolean) => void;
  setLoadingDeletePermanentMerchantTrashed: (value: boolean) => void;
  setLoadingRestoreAllMerchantTrashed: (value: boolean) => void;
  setLoadingDeletePermanentAllMerchantTrashed: (value: boolean) => void;

  setErrorGetMerchantsTrashed: (value: boolean) => void;
  setErrorRestoreMerchantTrashed: (value: boolean) => void;
  setErrorDeletePermanentMerchantTrashed: (value: boolean) => void;
  setErrorRestoreAllMerchantTrashed: (value: boolean) => void;
  setErrorDeletePermanentAllMerchantTrashed: (value: boolean) => void;

  findAllMerchantsTrashed: (req: FindAllMerchantTrashed) => Promise<void>;

  restoreMerchantTrashed: (req: RestoreMerchantTrashed) => Promise<boolean>;
  deletePermanentMerchant: (req: DeletePermanentMerchant) => Promise<boolean>;
  restoreMerchantAllTrashed: (toast: any) => Promise<boolean>;
  deletePermanentAllMerchant: (toast: any) => Promise<boolean>;
}
