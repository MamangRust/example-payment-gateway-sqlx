import { handleApiError } from "@/helpers/handleApi";
import { handleMessageAction } from "@/helpers/message";
import { getAccessToken } from "@/store/auth";
import {
  DeletePermanentTopup,
  FindAllTopupTrashed,
  RestoreTopupTrashed,
} from "@/types/domain/request/topup";
import TopupTrashedService from "@/services/api/topup/topup_trashed";
import TopupTrashedCommand from "@/services/ipc/topup/topup_trashed";
import { TopupTrashedStore } from "@/types/state";
import { create } from "zustand";
import { isTauri } from "@tauri-apps/api/core";

const useTopupTrashedStore = create<TopupTrashedStore>((set, _get) => ({
  topups: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingGetTopupsTrashed: false,
  loadingRestoreTopupTrashed: false,
  loadingDeletePermanentTopupTrashed: false,
  loadingRestoreAllTopupTrashed: false,
  loadingDeletePermanentAllTopupTrashed: false,

  errorGetTopupsTrashed: false,
  errorRestoreTopupTrashed: false,
  errorDeletePermanentTopupTrashed: false,
  errorRestoreAllTopupTrashed: false,
  errorDeletePermanentAllTopupTrashed: false,

  setLoadingGetTopupsTrashed: (value: boolean) =>
    set({ loadingGetTopupsTrashed: value }),
  setLoadingRestoreTopupTrashed: (value: boolean) =>
    set({ loadingRestoreTopupTrashed: value }),
  setLoadingDeletePermanentTopupTrashed: (value: boolean) =>
    set({ loadingDeletePermanentTopupTrashed: value }),
  setLoadingRestoreAllTopupTrashed: (value: boolean) =>
    set({ loadingRestoreAllTopupTrashed: value }),
  setLoadingDeletePermanentAllTopupTrashed: (value: boolean) =>
    set({ loadingDeletePermanentAllTopupTrashed: value }),

  setErrorGetTopupsTrashed: (value: boolean) =>
    set({ errorGetTopupsTrashed: value }),
  setErrorRestoreTopupTrashed: (value: boolean) =>
    set({ errorRestoreTopupTrashed: value }),
  setErrorDeletePermanentTopupTrashed: (value: boolean) =>
    set({ errorDeletePermanentTopupTrashed: value }),
  setErrorRestoreAllTopupTrashed: (value: boolean) =>
    set({ errorRestoreAllTopupTrashed: value }),
  setErrorDeletePermanentAllTopupTrashed: (value: boolean) =>
    set({ errorDeletePermanentAllTopupTrashed: value }),

  findAllTopupsTrashed: async (req: FindAllTopupTrashed) => {
    set({ loadingGetTopupsTrashed: true, errorGetTopupsTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupTrashedCommand.findAllTopupsTrashed(
          token,
          req,
        );

        set({
          topups: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTopupsTrashed: false,
          errorGetTopupsTrashed: null,
        });
      } else {
        const response = await TopupTrashedService.findAllTopupsTrashed(
          req,
          token,
        );

        set({
          topups: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTopupsTrashed: false,
          errorGetTopupsTrashed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetTopupsTrashed: false }),
        (message: any) => set({ errorGetTopupsTrashed: message }),
        req.toast,
      );
    }
  },

  restoreTopupTrashed: async (req: RestoreTopupTrashed) => {
    set({ loadingRestoreTopupTrashed: true, errorRestoreTopupTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TopupTrashedCommand.restoreTopupTrashed(token, req);

        set({
          loadingRestoreTopupTrashed: false,
          errorRestoreTopupTrashed: null,
        });
        handleMessageAction("topup", "restore");
      } else {
        await TopupTrashedService.restoreTopupTrashed(req, token);

        set({
          loadingRestoreTopupTrashed: false,
          errorRestoreTopupTrashed: null,
        });
        handleMessageAction("topup", "restore");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreTopupTrashed: false }),
        (message: any) => set({ errorRestoreTopupTrashed: message }),
        req.toast,
      );

      return false;
    }
  },

  deletePermanentTopup: async (req: DeletePermanentTopup) => {
    set({
      loadingDeletePermanentTopupTrashed: true,
      errorDeletePermanentTopupTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TopupTrashedCommand.deletePermanentTopup(token, req);

        set({
          loadingDeletePermanentTopupTrashed: false,
          errorDeletePermanentTopupTrashed: null,
        });
        handleMessageAction("topup", "deletePermanent");
      } else {
        await TopupTrashedService.deletePermanentTopup(req, token);

        set({
          loadingDeletePermanentTopupTrashed: false,
          errorDeletePermanentTopupTrashed: null,
        });
        handleMessageAction("topup", "deletePermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentTopupTrashed: false }),
        (message: any) => set({ errorDeletePermanentTopupTrashed: message }),
        req.toast,
      );

      return false;
    }
  },
  restoreTopupAllTrashed: async (toast: any) => {
    set({
      loadingRestoreAllTopupTrashed: true,
      errorRestoreAllTopupTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TopupTrashedCommand.restoreTopupAllTrashed(token);

        set({
          loadingRestoreAllTopupTrashed: false,
          errorRestoreAllTopupTrashed: null,
        });
        handleMessageAction("topup", "restoreAll");
      } else {
        await TopupTrashedService.restoreTopupAllTrashed(token);

        set({
          loadingRestoreAllTopupTrashed: false,
          errorRestoreAllTopupTrashed: null,
        });
        handleMessageAction("topup", "restoreAll");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreAllTopupTrashed: false }),
        (message: any) => set({ errorRestoreAllTopupTrashed: message }),
        toast,
      );
      return false;
    }
  },

  deletePermanentAllTopup: async (toast: any) => {
    set({
      loadingDeletePermanentAllTopupTrashed: true,
      errorDeletePermanentAllTopupTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TopupTrashedCommand.deletePermanentAllTopup(token);

        set({
          loadingDeletePermanentTopupTrashed: false,
          errorDeletePermanentTopupTrashed: null,
        });
        handleMessageAction("topup", "deleteAllPermanent");
      } else {
        await TopupTrashedService.deletePermanentAllTopup(token);

        set({
          loadingDeletePermanentTopupTrashed: false,
          errorDeletePermanentTopupTrashed: null,
        });
        handleMessageAction("topup", "deleteAllPermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentTopupTrashed: false }),
        (message: any) => set({ errorDeletePermanentTopupTrashed: message }),
        toast,
      );

      return false;
    }
  },
}));

export default useTopupTrashedStore;
