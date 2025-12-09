import { handleApiError } from "@/helpers/handleApi";
import { handleMessageAction } from "@/helpers/message";
import { getAccessToken } from "@/store/auth";
import { DeletePermanentMerchant } from "@/types/domain/request/merchant/trashed/delete";
import { FindAllMerchantTrashed } from "@/types/domain/request/merchant/trashed/list";
import { RestoreMerchantTrashed } from "@/types/domain/request/merchant/trashed/restore";
import { MerchantTrashedStore } from "@/types/state";

import MerchantTrashedCommand from "@/services/ipc/merchant/merchant_trashed";
import MerchantTrashedService from "@/services/api/merchant/merchant_trashed";
import { create } from "zustand";
import { isTauri } from "@tauri-apps/api/core";

const useMerchantTrashedStore = create<MerchantTrashedStore>((set, _get) => ({
  merchants: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingGetMerchantsTrashed: false,
  loadingRestoreMerchantTrashed: false,
  loadingDeletePermanentMerchantTrashed: false,
  loadingRestoreAllMerchantTrashed: false,
  loadingDeletePermanentAllMerchantTrashed: false,

  errorGetMerchantsTrashed: false,
  errorRestoreMerchantTrashed: false,
  errorDeletePermanentMerchantTrashed: false,
  errorRestoreAllMerchantTrashed: false,
  errorDeletePermanentAllMerchantTrashed: false,

  setLoadingGetMerchantsTrashed: (value: boolean) =>
    set({ loadingGetMerchantsTrashed: value }),
  setLoadingRestoreMerchantTrashed: (value: boolean) =>
    set({ loadingRestoreMerchantTrashed: value }),
  setLoadingDeletePermanentMerchantTrashed: (value: boolean) =>
    set({ loadingDeletePermanentMerchantTrashed: value }),
  setLoadingRestoreAllMerchantTrashed: (value: boolean) =>
    set({ loadingRestoreAllMerchantTrashed: value }),
  setLoadingDeletePermanentAllMerchantTrashed: (value: boolean) =>
    set({ loadingDeletePermanentAllMerchantTrashed: value }),

  setErrorGetMerchantsTrashed: (value: boolean) =>
    set({ errorGetMerchantsTrashed: value }),
  setErrorRestoreMerchantTrashed: (value: boolean) =>
    set({ errorRestoreMerchantTrashed: value }),
  setErrorDeletePermanentMerchantTrashed: (value: boolean) =>
    set({ errorDeletePermanentMerchantTrashed: value }),
  setErrorRestoreAllMerchantTrashed: (value: boolean) =>
    set({ errorRestoreAllMerchantTrashed: value }),
  setErrorDeletePermanentAllMerchantTrashed: (value: boolean) =>
    set({ errorDeletePermanentAllMerchantTrashed: value }),

  findAllMerchantsTrashed: async (req: FindAllMerchantTrashed) => {
    set({ loadingGetMerchantsTrashed: true, errorGetMerchantsTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantTrashedCommand.findAllMerchantsTrashed(
          token,
          req,
        );
        set({
          merchants: response.data,
          pagination: {
            currentPage: response.pagination!.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination!.total_records,
            totalPages: response.pagination!.total_pages,
          },
          loadingGetMerchantsTrashed: false,
          errorGetMerchantsTrashed: null,
        });
      } else {
        const response = await MerchantTrashedService.findAllMerchantsTrashed(
          req,
          token,
        );
        set({
          merchants: response.data,
          pagination: {
            currentPage: response.pagination!.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination!.total_records,
            totalPages: response.pagination!.total_pages,
          },
          loadingGetMerchantsTrashed: false,
          errorGetMerchantsTrashed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetMerchantsTrashed: false }),
        (message: any) => set({ errorGetMerchantsTrashed: message }),
        req.toast,
      );
    }
  },

  restoreMerchantTrashed: async (req: RestoreMerchantTrashed) => {
    set({
      loadingRestoreMerchantTrashed: true,
      errorRestoreMerchantTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await MerchantTrashedCommand.restoreMerchantTrashed(token, req);

        set({
          loadingRestoreMerchantTrashed: false,
          errorRestoreMerchantTrashed: null,
        });

        handleMessageAction("merchant", "restore");
      } else {
        await MerchantTrashedService.restoreMerchantTrashed(req, token);

        set({
          loadingRestoreMerchantTrashed: false,
          errorRestoreMerchantTrashed: null,
        });

        handleMessageAction("merchant", "restore");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreMerchantTrashed: false }),
        (message: any) => set({ errorRestoreMerchantTrashed: message }),
        req.toast,
      );

      return false;
    }
  },

  deletePermanentMerchant: async (req: DeletePermanentMerchant) => {
    set({
      loadingDeletePermanentMerchantTrashed: true,
      errorDeletePermanentMerchantTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await MerchantTrashedCommand.deletePermanentMerchant(token, req);

        set({
          loadingDeletePermanentMerchantTrashed: false,
          errorDeletePermanentMerchantTrashed: null,
        });
        handleMessageAction("merchant", "deletePermanent");
      } else {
        await MerchantTrashedService.deletePermanentMerchant(req, token);

        set({
          loadingDeletePermanentMerchantTrashed: false,
          errorDeletePermanentMerchantTrashed: null,
        });
        handleMessageAction("merchant", "deletePermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentMerchantTrashed: false }),
        (message: any) => set({ errorDeletePermanentMerchantTrashed: message }),
        req.toast,
      );

      return false;
    }
  },
  restoreMerchantAllTrashed: async (toast: any) => {
    set({
      loadingRestoreAllMerchantTrashed: true,
      errorRestoreAllMerchantTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await MerchantTrashedCommand.restoreMerchantAllTrashed(token);

        set({
          loadingRestoreAllMerchantTrashed: false,
          errorRestoreAllMerchantTrashed: null,
        });
        handleMessageAction("merchant", "restoreAll");
      } else {
        await MerchantTrashedService.restoreMerchantAllTrashed(token);

        set({
          loadingRestoreAllMerchantTrashed: false,
          errorRestoreAllMerchantTrashed: null,
        });
        handleMessageAction("merchant", "restoreAll");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreAllMerchantTrashed: false }),
        (message: any) => set({ errorRestoreAllMerchantTrashed: message }),
        toast,
      );

      return false;
    }
  },

  deletePermanentAllMerchant: async (toast: any) => {
    set({
      loadingDeletePermanentAllMerchantTrashed: true,
      errorDeletePermanentAllMerchantTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await MerchantTrashedCommand.deletePermanentAllMerchant(token);
        set({
          loadingDeletePermanentMerchantTrashed: false,
          errorDeletePermanentMerchantTrashed: null,
        });
        handleMessageAction("merchant", "deleteAllPermanent");
      } else {
        await MerchantTrashedService.deletePermanentAllMerchant(token);
        set({
          loadingDeletePermanentMerchantTrashed: false,
          errorDeletePermanentMerchantTrashed: null,
        });
        handleMessageAction("merchant", "deleteAllPermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentMerchantTrashed: false }),
        (message: any) => set({ errorDeletePermanentMerchantTrashed: message }),
        toast,
      );
      return false;
    }
  },
}));

export default useMerchantTrashedStore;
