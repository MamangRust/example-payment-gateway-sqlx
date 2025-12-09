import { handleApiError } from "@/helpers/handleApi";
import { handleMessageAction } from "@/helpers/message";
import { getAccessToken } from "@/store/auth";
import {
  DeletePermanentSaldo,
  FindAllSaldoTrashed,
  RestoreSaldoTrashed,
} from "@/types/domain/request";

import SaldoTrashedCommand from "@/services/ipc/saldo/saldo_trashed";
import SaldoTrashedService from "@/services/api/saldo/saldo_trashed";
import { SaldoTrashedStore } from "@/types/state";
import { create } from "zustand";
import { isTauri } from "@tauri-apps/api/core";

const useSaldoTrashedStore = create<SaldoTrashedStore>((set, _get) => ({
  saldos: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingGetSaldosTrashed: false,
  loadingRestoreSaldoTrashed: false,
  loadingDeletePermanentSaldoTrashed: false,
  loadingRestoreAllSaldoTrashed: false,
  loadingDeletePermanentAllSaldoTrashed: false,

  errorGetSaldosTrashed: false,
  errorRestoreSaldoTrashed: false,
  errorDeletePermanentSaldoTrashed: false,
  errorRestoreAllSaldoTrashed: false,
  errorDeletePermanentAllSaldoTrashed: false,

  setLoadingGetSaldosTrashed: (value: boolean) =>
    set({ loadingGetSaldosTrashed: value }),
  setLoadingRestoreSaldoTrashed: (value: boolean) =>
    set({ loadingRestoreSaldoTrashed: value }),
  setLoadingDeletePermanentSaldoTrashed: (value: boolean) =>
    set({ loadingDeletePermanentSaldoTrashed: value }),
  setLoadingRestoreAllSaldoTrashed: (value: boolean) =>
    set({ loadingRestoreAllSaldoTrashed: value }),
  setLoadingDeletePermanentAllSaldoTrashed: (value: boolean) =>
    set({ loadingDeletePermanentAllSaldoTrashed: value }),

  setErrorGetSaldosTrashed: (value: boolean) =>
    set({ errorGetSaldosTrashed: value }),
  setErrorRestoreSaldoTrashed: (value: boolean) =>
    set({ errorRestoreSaldoTrashed: value }),
  setErrorDeletePermanentSaldoTrashed: (value: boolean) =>
    set({ errorDeletePermanentSaldoTrashed: value }),
  setErrorRestoreAllSaldoTrashed: (value: boolean) =>
    set({ errorRestoreAllSaldoTrashed: value }),
  setErrorDeletePermanentAllSaldoTrashed: (value: boolean) =>
    set({ errorDeletePermanentAllSaldoTrashed: value }),

  findAllSaldosTrashed: async (req: FindAllSaldoTrashed) => {
    set({ loadingGetSaldosTrashed: true, errorGetSaldosTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await SaldoTrashedCommand.findAllSaldosTrashed(
          token,
          req,
        );

        set({
          saldos: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetSaldosTrashed: false,
          errorGetSaldosTrashed: null,
        });
      } else {
        const response = await SaldoTrashedService.findAllSaldosTrashed(
          req,
          token,
        );

        set({
          saldos: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetSaldosTrashed: false,
          errorGetSaldosTrashed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetSaldosTrashed: false }),
        (message: any) => set({ errorGetSaldosTrashed: message }),
        req.toast,
      );
    }
  },

  restoreSaldoTrashed: async (req: RestoreSaldoTrashed) => {
    set({ loadingRestoreSaldoTrashed: true, errorRestoreSaldoTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await SaldoTrashedCommand.restoreSaldoTrashed(token, req);

        set({
          loadingRestoreSaldoTrashed: false,
          errorRestoreSaldoTrashed: null,
        });
        handleMessageAction("saldo", "restore");
      } else {
        await SaldoTrashedService.restoreSaldoTrashed(req, token);

        set({
          loadingRestoreSaldoTrashed: false,
          errorRestoreSaldoTrashed: null,
        });
        handleMessageAction("saldo", "restore");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreSaldoTrashed: false }),
        (message: any) => set({ errorRestoreSaldoTrashed: message }),
        req.toast,
      );

      return false;
    }
  },

  deletePermanentSaldo: async (req: DeletePermanentSaldo) => {
    set({
      loadingDeletePermanentSaldoTrashed: true,
      errorDeletePermanentSaldoTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await SaldoTrashedCommand.deletePermanentSaldo(token, req);

        set({
          loadingDeletePermanentSaldoTrashed: false,
          errorDeletePermanentSaldoTrashed: null,
        });
        handleMessageAction("saldo", "deletePermanent");
      } else {
        await SaldoTrashedService.deletePermanentSaldo(req, token);

        set({
          loadingDeletePermanentSaldoTrashed: false,
          errorDeletePermanentSaldoTrashed: null,
        });
        handleMessageAction("saldo", "deletePermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentSaldoTrashed: false }),
        (message: any) => set({ errorDeletePermanentSaldoTrashed: message }),
        req.toast,
      );

      return false;
    }
  },
  restoreSaldoAllTrashed: async (toast: any) => {
    set({
      loadingRestoreAllSaldoTrashed: true,
      errorRestoreAllSaldoTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await SaldoTrashedCommand.restoreSaldoAllTrashed(token);

        set({
          loadingRestoreAllSaldoTrashed: false,
          errorRestoreAllSaldoTrashed: null,
        });

        handleMessageAction("saldo", "restoreAll");
      } else {
        await SaldoTrashedService.restoreSaldoAllTrashed(token);

        set({
          loadingRestoreAllSaldoTrashed: false,
          errorRestoreAllSaldoTrashed: null,
        });

        handleMessageAction("saldo", "restoreAll");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreAllSaldoTrashed: false }),
        (message: any) => set({ errorRestoreAllSaldoTrashed: message }),
        toast,
      );

      return false;
    }
  },

  deletePermanentAllSaldo: async (toast: any) => {
    set({
      loadingDeletePermanentAllSaldoTrashed: true,
      errorDeletePermanentAllSaldoTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await SaldoTrashedCommand.deletePermanentAllSaldo(token);

        set({
          loadingDeletePermanentSaldoTrashed: false,
          errorDeletePermanentSaldoTrashed: null,
        });

        handleMessageAction("saldo", "deleteAllPermanent");
      } else {
        await SaldoTrashedService.deletePermanentAllSaldo(token);

        set({
          loadingDeletePermanentSaldoTrashed: false,
          errorDeletePermanentSaldoTrashed: null,
        });

        handleMessageAction("saldo", "deleteAllPermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentSaldoTrashed: false }),
        (message: any) => set({ errorDeletePermanentSaldoTrashed: message }),
        toast,
      );

      return false;
    }
  },
}));

export default useSaldoTrashedStore;
