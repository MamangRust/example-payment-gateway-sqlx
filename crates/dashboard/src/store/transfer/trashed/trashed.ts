import { handleApiError } from "@/helpers/handleApi";
import { handleMessageAction } from "@/helpers/message";
import { getAccessToken } from "@/store/auth";
import {
  DeletePermanentTransfer,
  FindAllTransferTrashed,
  RestoreTransferTrashed,
} from "@/types/domain/request";
import TransferTrashedService from "@/services/api/transfer/transfer_trashed";
import TransferTrashedCommand from "@/services/ipc/transfer/transfer_trashed";
import { TransferTrashedStore } from "@/types/state";
import { create } from "zustand";
import { isTauri } from "@tauri-apps/api/core";

const useTransferTrashedStore = create<TransferTrashedStore>((set, _get) => ({
  transfers: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingGetTransfersTrashed: false,
  loadingRestoreTransferTrashed: false,
  loadingDeletePermanentTransferTrashed: false,
  loadingRestoreAllTransferTrashed: false,
  loadingDeletePermanentAllTransferTrashed: false,

  errorGetTransfersTrashed: false,
  errorRestoreTransferTrashed: false,
  errorDeletePermanentTransferTrashed: false,
  errorRestoreAllTransferTrashed: false,
  errorDeletePermanentAllTransferTrashed: false,

  setLoadingGetTransfersTrashed: (value: boolean) =>
    set({ loadingGetTransfersTrashed: value }),
  setLoadingRestoreTransferTrashed: (value: boolean) =>
    set({ loadingRestoreTransferTrashed: value }),
  setLoadingDeletePermanentTransferTrashed: (value: boolean) =>
    set({ loadingDeletePermanentTransferTrashed: value }),
  setLoadingRestoreAllTransferTrashed: (value: boolean) =>
    set({ loadingRestoreAllTransferTrashed: value }),
  setLoadingDeletePermanentAllTransferTrashed: (value: boolean) =>
    set({ loadingDeletePermanentAllTransferTrashed: value }),

  setErrorGetTransfersTrashed: (value: boolean) =>
    set({ errorGetTransfersTrashed: value }),
  setErrorRestoreTransferTrashed: (value: boolean) =>
    set({ errorRestoreTransferTrashed: value }),
  setErrorDeletePermanentTransferTrashed: (value: boolean) =>
    set({ errorDeletePermanentTransferTrashed: value }),
  setErrorRestoreAllTransferTrashed: (value: boolean) =>
    set({ errorRestoreAllTransferTrashed: value }),
  setErrorDeletePermanentAllTransferTrashed: (value: boolean) =>
    set({ errorDeletePermanentAllTransferTrashed: value }),

  findAllTransfersTrashed: async (req: FindAllTransferTrashed) => {
    set({ loadingGetTransfersTrashed: true, errorGetTransfersTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferTrashedCommand.findAllTransfersTrashed(
          token,
          req,
        );

        set({
          transfers: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTransfersTrashed: false,
          errorGetTransfersTrashed: null,
        });
      } else {
        const response = await TransferTrashedService.findAllTransferssTrashed(
          req,
          token,
        );

        set({
          transfers: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTransfersTrashed: false,
          errorGetTransfersTrashed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetTransfersTrashed: false }),
        (message: any) => set({ errorGetTransfersTrashed: message }),
        req.toast,
      );
    }
  },

  restoreTransferTrashed: async (req: RestoreTransferTrashed) => {
    set({
      loadingRestoreTransferTrashed: true,
      errorRestoreTransferTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TransferTrashedCommand.restoreTransferTrashed(token, req);
        set({
          loadingRestoreTransferTrashed: false,
          errorRestoreTransferTrashed: null,
        });
        handleMessageAction("transfer", "restore");
      } else {
        await TransferTrashedService.restoreTransfersTrashed(req, token);
        set({
          loadingRestoreTransferTrashed: false,
          errorRestoreTransferTrashed: null,
        });
        handleMessageAction("transfer", "restore");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreTransferTrashed: false }),
        (message: any) => set({ errorRestoreTransferTrashed: message }),
        req.toast,
      );
      return false;
    }
  },

  deletePermanentTransfer: async (req: DeletePermanentTransfer) => {
    set({
      loadingDeletePermanentTransferTrashed: true,
      errorDeletePermanentTransferTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TransferTrashedCommand.deletePermanentTransfer(token, req);
        set({
          loadingDeletePermanentTransferTrashed: false,
          errorDeletePermanentTransferTrashed: null,
        });
        handleMessageAction("transfer", "deletePermanent");
      } else {
        await TransferTrashedService.deletePermanentTransfer(req, token);
        set({
          loadingDeletePermanentTransferTrashed: false,
          errorDeletePermanentTransferTrashed: null,
        });
        handleMessageAction("transfer", "deletePermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentTransferTrashed: false }),
        (message: any) => set({ errorDeletePermanentTransferTrashed: message }),
        req.toast,
      );

      return false;
    }
  },
  restoreTransferAllTrashed: async (toast: any) => {
    set({
      loadingRestoreAllTransferTrashed: true,
      errorRestoreAllTransferTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TransferTrashedCommand.restoreTransferAllTrashed(token);

        set({
          loadingRestoreAllTransferTrashed: false,
          errorRestoreAllTransferTrashed: null,
        });
        handleMessageAction("transfer", "restoreAll");
      } else {
        await TransferTrashedService.restoreTransfersAllTrashed(token);

        set({
          loadingRestoreAllTransferTrashed: false,
          errorRestoreAllTransferTrashed: null,
        });
        handleMessageAction("transfer", "restoreAll");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreAllTransferTrashed: false }),
        (message: any) => set({ errorRestoreAllTransferTrashed: message }),
        toast,
      );

      return false;
    }
  },

  deletePermanentAllTransfer: async (toast: any) => {
    set({
      loadingDeletePermanentAllTransferTrashed: true,
      errorDeletePermanentAllTransferTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TransferTrashedCommand.deletePermanentAllTransfer(token);

        set({
          loadingDeletePermanentTransferTrashed: false,
          errorDeletePermanentTransferTrashed: null,
        });

        handleMessageAction("transfer", "deleteAllPermanent");
      } else {
        await TransferTrashedService.deletePermanentAllTransfers(token);

        set({
          loadingDeletePermanentTransferTrashed: false,
          errorDeletePermanentTransferTrashed: null,
        });

        handleMessageAction("transfer", "deleteAllPermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentTransferTrashed: false }),
        (message: any) => set({ errorDeletePermanentTransferTrashed: message }),
        toast,
      );
      return false;
    }
  },
}));

export default useTransferTrashedStore;
