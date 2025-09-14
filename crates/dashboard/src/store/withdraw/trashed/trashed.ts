import { handleApiError } from "@/helpers/handleApi";
import { handleMessageAction } from "@/helpers/message";
import { getAccessToken } from "@/store/auth";
import {
  DeletePermanentWithdraw,
  FindAllWithdrawTrashed,
  RestoreWithdrawTrashed,
} from "@/types/domain/request";
import WithdrawTrashedCommand from "@/services/ipc/withdraw/withdraw_trashed";
import WithdrawTrashedService from "@/services/api/withdraw/withdraw_trashed";
import { WithdrawTrashedStore } from "@/types/state";
import { create } from "zustand";
import { isTauri } from "@tauri-apps/api/core";

const useWithdrawTrashedStore = create<WithdrawTrashedStore>((set, _get) => ({
  withdraws: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingGetWithdrawsTrashed: false,
  loadingRestoreWithdrawTrashed: false,
  loadingDeletePermanentWithdrawTrashed: false,
  loadingRestoreAllWithdrawTrashed: false,
  loadingDeletePermanentAllWithdrawTrashed: false,

  errorGetWithdrawsTrashed: false,
  errorRestoreWithdrawTrashed: false,
  errorDeletePermanentWithdrawTrashed: false,
  errorRestoreAllWithdrawTrashed: false,
  errorDeletePermanentAllWithdrawTrashed: false,

  setLoadingGetWithdrawsTrashed: (value: boolean) =>
    set({ loadingGetWithdrawsTrashed: value }),
  setLoadingRestoreWithdrawTrashed: (value: boolean) =>
    set({ loadingRestoreWithdrawTrashed: value }),
  setLoadingDeletePermanentWithdrawTrashed: (value: boolean) =>
    set({ loadingDeletePermanentWithdrawTrashed: value }),
  setLoadingRestoreAllWithdrawTrashed: (value: boolean) =>
    set({ loadingRestoreAllWithdrawTrashed: value }),
  setLoadingDeletePermanentAllWithdrawTrashed: (value: boolean) =>
    set({ loadingDeletePermanentAllWithdrawTrashed: value }),

  setErrorGetWithdrawsTrashed: (value: boolean) =>
    set({ errorGetWithdrawsTrashed: value }),
  setErrorRestoreWithdrawTrashed: (value: boolean) =>
    set({ errorRestoreWithdrawTrashed: value }),
  setErrorDeletePermanentWithdrawTrashed: (value: boolean) =>
    set({ errorDeletePermanentWithdrawTrashed: value }),
  setErrorRestoreAllWithdrawTrashed: (value: boolean) =>
    set({ errorRestoreAllWithdrawTrashed: value }),
  setErrorDeletePermanentAllWithdrawTrashed: (value: boolean) =>
    set({ errorDeletePermanentAllWithdrawTrashed: value }),

  findAllWithdrawsTrashed: async (req: FindAllWithdrawTrashed) => {
    set({ loadingGetWithdrawsTrashed: true, errorGetWithdrawsTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawTrashedCommand.findAllWithdrawsTrashed(
          token,
          req,
        );
        set({
          withdraws: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetWithdrawsTrashed: false,
          errorGetWithdrawsTrashed: null,
        });
      } else {
        const response = await WithdrawTrashedService.findAllWithdrawsTrashed(
          req,
          token,
        );
        set({
          withdraws: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetWithdrawsTrashed: false,
          errorGetWithdrawsTrashed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetWithdrawsTrashed: false }),
        (message: any) => set({ errorGetWithdrawsTrashed: message }),
        req.toast,
      );
    }
  },

  restoreWithdrawTrashed: async (req: RestoreWithdrawTrashed) => {
    set({
      loadingRestoreWithdrawTrashed: true,
      errorRestoreWithdrawTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await WithdrawTrashedCommand.restoreWithdrawTrashed(token, req);
        set({
          loadingRestoreWithdrawTrashed: false,
          errorRestoreWithdrawTrashed: null,
        });
        handleMessageAction("withdraw", "restore");
      } else {
        await WithdrawTrashedService.restoreWithdrawTrashed(req, token);
        set({
          loadingRestoreWithdrawTrashed: false,
          errorRestoreWithdrawTrashed: null,
        });
        handleMessageAction("withdraw", "restore");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreWithdrawTrashed: false }),
        (message: any) => set({ errorRestoreWithdrawTrashed: message }),
        req.toast,
      );

      return false;
    }
  },

  deletePermanentWithdraw: async (req: DeletePermanentWithdraw) => {
    set({
      loadingDeletePermanentWithdrawTrashed: true,
      errorDeletePermanentWithdrawTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await WithdrawTrashedCommand.deletePermanentWithdraw(token, req);
        set({
          loadingDeletePermanentWithdrawTrashed: false,
          errorDeletePermanentWithdrawTrashed: null,
        });
        handleMessageAction("withdraw", "deletePermanent");
      } else {
        await WithdrawTrashedService.deletePermanentWithdraw(req, token);
        set({
          loadingDeletePermanentWithdrawTrashed: false,
          errorDeletePermanentWithdrawTrashed: null,
        });
        handleMessageAction("withdraw", "deletePermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentWithdrawTrashed: false }),
        (message: any) => set({ errorDeletePermanentWithdrawTrashed: message }),
        req.toast,
      );
      return false;
    }
  },
  restoreWithdrawAllTrashed: async (toast: any) => {
    set({
      loadingRestoreAllWithdrawTrashed: true,
      errorRestoreAllWithdrawTrashed: false,
    });

    try {
      const token = getAccessToken();
      if (isTauri()) {
        await WithdrawTrashedCommand.restoreWithdrawAllTrashed(token);

        set({
          loadingRestoreAllWithdrawTrashed: false,
          errorRestoreAllWithdrawTrashed: null,
        });
        handleMessageAction("withdraw", "restoreAll");
      } else {
        await WithdrawTrashedService.restoreWithdrawAllTrashed(token);

        set({
          loadingRestoreAllWithdrawTrashed: false,
          errorRestoreAllWithdrawTrashed: null,
        });
        handleMessageAction("withdraw", "restoreAll");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreAllWithdrawTrashed: false }),
        (message: any) => set({ errorRestoreAllWithdrawTrashed: message }),
        toast,
      );

      return false;
    }
  },

  deletePermanentAllWithdraw: async (toast: any) => {
    set({
      loadingDeletePermanentAllWithdrawTrashed: true,
      errorDeletePermanentAllWithdrawTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await WithdrawTrashedCommand.deletePermanentAllWithdraw(token);

        set({
          loadingDeletePermanentWithdrawTrashed: false,
          errorDeletePermanentWithdrawTrashed: null,
        });
        handleMessageAction("withdraw", "deleteAllPermanent");
      } else {
        await WithdrawTrashedService.deletePermanentAllWithdraw(token);

        set({
          loadingDeletePermanentWithdrawTrashed: false,
          errorDeletePermanentWithdrawTrashed: null,
        });
        handleMessageAction("withdraw", "deleteAllPermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentWithdrawTrashed: false }),
        (message: any) => set({ errorDeletePermanentWithdrawTrashed: message }),
        toast,
      );

      return false;
    }
  },
}));

export default useWithdrawTrashedStore;
