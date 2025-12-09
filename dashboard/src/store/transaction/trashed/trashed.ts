import { handleApiError } from "@/helpers/handleApi";
import { handleMessageAction } from "@/helpers/message";
import { getAccessToken } from "@/store/auth";
import {
  DeletePermanentTransaction,
  FindAllTransactionTrashed,
  RestoreTransactionTrashed,
} from "@/types/domain/request";
import TransactionTrashedService from "@/services/api/transaction/transaction_trashed";
import TransactionTrashedCommand from "@/services/ipc/transaction/transaction_trashed";
import { TransactionTrashedStore } from "@/types/state";
import { create } from "zustand";
import { isTauri } from "@tauri-apps/api/core";

const useTransactionTrashedStore = create<TransactionTrashedStore>(
  (set, _get) => ({
    transactions: null,

    pagination: {
      currentPage: 1,
      page_size: 10,
      totalItems: 0,
      totalPages: 0,
    },

    loadingGetTransactionsTrashed: false,
    loadingRestoreTransactionTrashed: false,
    loadingDeletePermanentTransactionTrashed: false,
    loadingRestoreAllTransactionTrashed: false,
    loadingDeletePermanentAllTransactionTrashed: false,

    errorGetTransactionsTrashed: false,
    errorRestoreTransactionTrashed: false,
    errorDeletePermanentTransactionTrashed: false,
    errorRestoreAllTransactionTrashed: false,
    errorDeletePermanentAllTransactionTrashed: false,

    setLoadingGetTransactionsTrashed: (value: boolean) =>
      set({ loadingGetTransactionsTrashed: value }),
    setLoadingRestoreTransactionTrashed: (value: boolean) =>
      set({ loadingRestoreTransactionTrashed: value }),
    setLoadingDeletePermanentTransactionTrashed: (value: boolean) =>
      set({ loadingDeletePermanentTransactionTrashed: value }),
    setLoadingRestoreAllTransactionTrashed: (value: boolean) =>
      set({ loadingRestoreAllTransactionTrashed: value }),
    setLoadingDeletePermanentAllTransactionTrashed: (value: boolean) =>
      set({ loadingDeletePermanentAllTransactionTrashed: value }),

    setErrorGetTransactionsTrashed: (value: boolean) =>
      set({ errorGetTransactionsTrashed: value }),
    setErrorRestoreTransactionTrashed: (value: boolean) =>
      set({ errorRestoreTransactionTrashed: value }),
    setErrorDeletePermanentTransactionTrashed: (value: boolean) =>
      set({ errorDeletePermanentTransactionTrashed: value }),
    setErrorRestoreAllTransactionTrashed: (value: boolean) =>
      set({ errorRestoreAllTransactionTrashed: value }),
    setErrorDeletePermanentAllTransactionTrashed: (value: boolean) =>
      set({ errorDeletePermanentAllTransactionTrashed: value }),

    findAllTransactionsTrashed: async (req: FindAllTransactionTrashed) => {
      set({
        loadingGetTransactionsTrashed: true,
        errorGetTransactionsTrashed: null,
      });
      try {
        const token = getAccessToken();

        if (isTauri()) {
          const response =
            await TransactionTrashedCommand.findAllTransactionsTrashed(
              token,
              req,
            );

          set({
            transactions: response.data,
            pagination: {
              currentPage: response.pagination.current_page,
              page_size: response.pagination.page_size,
              totalItems: response.pagination.total_records,
              totalPages: response.pagination.total_pages,
            },
            loadingGetTransactionsTrashed: false,
            errorGetTransactionsTrashed: null,
          });
        } else {
          const response =
            await TransactionTrashedService.findAllTransactionsTrashed(
              req,
              token,
            );

          set({
            transactions: response.data,
            pagination: {
              currentPage: response.pagination.current_page,
              page_size: response.pagination.page_size,
              totalItems: response.pagination.total_records,
              totalPages: response.pagination.total_pages,
            },
            loadingGetTransactionsTrashed: false,
            errorGetTransactionsTrashed: null,
          });
        }
      } catch (err) {
        handleApiError(
          err,
          () => set({ loadingGetTransactionsTrashed: false }),
          (message: any) => set({ errorGetTransactionsTrashed: message }),
          req.toast,
        );
      }
    },

    restoreTransactionTrashed: async (req: RestoreTransactionTrashed) => {
      set({
        loadingRestoreTransactionTrashed: true,
        errorRestoreTransactionTrashed: null,
      });
      try {
        const token = getAccessToken();

        if (isTauri()) {
          await TransactionTrashedCommand.restoreTransactionTrashed(token, req);
          set({
            loadingRestoreTransactionTrashed: false,
            errorRestoreTransactionTrashed: null,
          });
          handleMessageAction("transaction", "restore");
        } else {
          await TransactionTrashedService.restoreTransactionTrashed(req, token);
          set({
            loadingRestoreTransactionTrashed: false,
            errorRestoreTransactionTrashed: null,
          });
          handleMessageAction("transaction", "restore");
        }

        return true;
      } catch (err) {
        handleApiError(
          err,
          () => set({ loadingRestoreTransactionTrashed: false }),
          (message: any) => set({ errorRestoreTransactionTrashed: message }),
          req.toast,
        );
        return false;
      }
    },

    deletePermanentTransaction: async (req: DeletePermanentTransaction) => {
      set({
        loadingDeletePermanentTransactionTrashed: true,
        errorDeletePermanentTransactionTrashed: null,
      });
      try {
        const token = getAccessToken();

        if (isTauri()) {
          await TransactionTrashedCommand.deletePermanentTransaction(
            token,
            req,
          );

          set({
            loadingDeletePermanentTransactionTrashed: false,
            errorDeletePermanentTransactionTrashed: null,
          });
          handleMessageAction("transaction", "deletePermanent");
        } else {
          await TransactionTrashedService.deletePermanentTransaction(
            req,
            token,
          );

          set({
            loadingDeletePermanentTransactionTrashed: false,
            errorDeletePermanentTransactionTrashed: null,
          });
          handleMessageAction("transaction", "deletePermanent");
        }

        return true;
      } catch (err) {
        handleApiError(
          err,
          () => set({ loadingDeletePermanentTransactionTrashed: false }),
          (message: any) =>
            set({ errorDeletePermanentTransactionTrashed: message }),
          req.toast,
        );
        return false;
      }
    },
    restoreTransactionAllTrashed: async (toast: any) => {
      set({
        loadingRestoreAllTransactionTrashed: true,
        errorRestoreAllTransactionTrashed: false,
      });

      try {
        const token = getAccessToken();

        if (isTauri()) {
          await TransactionTrashedCommand.restoreTransactionAllTrashed(token);

          set({
            loadingRestoreAllTransactionTrashed: false,
            errorRestoreAllTransactionTrashed: null,
          });
          handleMessageAction("transaction", "restoreAll");
        } else {
          await TransactionTrashedService.restoreTransactionAllTrashed(token);

          set({
            loadingRestoreAllTransactionTrashed: false,
            errorRestoreAllTransactionTrashed: null,
          });
          handleMessageAction("transaction", "restoreAll");
        }

        return true;
      } catch (err) {
        handleApiError(
          err,
          () => set({ loadingRestoreAllTransactionTrashed: false }),
          (message: any) => set({ errorRestoreAllTransactionTrashed: message }),
          toast,
        );
        return false;
      }
    },

    deletePermanentAllTransaction: async (toast: any) => {
      set({
        loadingDeletePermanentAllTransactionTrashed: true,
        errorDeletePermanentAllTransactionTrashed: false,
      });

      try {
        const token = getAccessToken();

        if (isTauri()) {
          await TransactionTrashedCommand.deletePermanentAllTransaction(token);

          set({
            loadingDeletePermanentTransactionTrashed: false,
            errorDeletePermanentTransactionTrashed: null,
          });
          handleMessageAction("transaction", "deleteAllPermanent");
        } else {
          await TransactionTrashedService.deletePermanentAllTransaction(token);

          set({
            loadingDeletePermanentTransactionTrashed: false,
            errorDeletePermanentTransactionTrashed: null,
          });
          handleMessageAction("transaction", "deleteAllPermanent");
        }

        return true;
      } catch (err) {
        handleApiError(
          err,
          () => set({ loadingDeletePermanentTransactionTrashed: false }),
          (message: any) =>
            set({ errorDeletePermanentTransactionTrashed: message }),
          toast,
        );

        return false;
      }
    },
  }),
);

export default useTransactionTrashedStore;
