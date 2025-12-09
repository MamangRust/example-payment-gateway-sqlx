import { handleApiError } from "@/helpers/handleApi";
import { handleMessageAction } from "@/helpers/message";
import { getAccessToken } from "@/store/auth";
import {
  DeletePermanentUser,
  FindAllUserTrashed,
  RestoreUserTrashed,
} from "@/types/domain/request";
import { UserTrashedStore } from "@/types/state";
import { create } from "zustand";
import UserTrashedCommand from "@/services/ipc/user/user_trashed";
import UserTrashedService from "@/services/api/user/user_trashed";
import { isTauri } from "@tauri-apps/api/core";

const useUserTrashedStore = create<UserTrashedStore>((set, _get) => ({
  users: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingGetUsersTrashed: false,
  loadingRestoreUserTrashed: false,
  loadingDeletePermanentUserTrashed: false,
  loadingRestoreAllUserTrashed: false,
  loadingDeletePermanentAllUserTrashed: false,

  errorGetUsersTrashed: false,
  errorRestoreUserTrashed: false,
  errorDeletePermanentUserTrashed: false,
  errorRestoreAllUserTrashed: false,
  errorDeletePermanentAllUserTrashed: false,

  setLoadingGetUsersTrashed: (value: boolean) =>
    set({ loadingGetUsersTrashed: value }),
  setLoadingRestoreUserTrashed: (value: boolean) =>
    set({ loadingRestoreUserTrashed: value }),
  setLoadingDeletePermanentUserTrashed: (value: boolean) =>
    set({ loadingDeletePermanentUserTrashed: value }),
  setLoadingRestoreAllUserTrashed: (value: boolean) =>
    set({ loadingRestoreAllUserTrashed: value }),
  setLoadingDeletePermanentAllUserTrashed: (value: boolean) =>
    set({ loadingDeletePermanentAllUserTrashed: value }),

  setErrorGetUsersTrashed: (value: boolean) =>
    set({ errorGetUsersTrashed: value }),
  setErrorRestoreUserTrashed: (value: boolean) =>
    set({ errorRestoreUserTrashed: value }),
  setErrorDeletePermanentUserTrashed: (value: boolean) =>
    set({ errorDeletePermanentUserTrashed: value }),
  setErrorRestoreAllUserTrashed: (value: boolean) =>
    set({ errorRestoreAllUserTrashed: value }),
  setErrorDeletePermanentAllUserTrashed: (value: boolean) =>
    set({ errorDeletePermanentAllUserTrashed: value }),

  findAllUsersTrashed: async (req: FindAllUserTrashed) => {
    set({ loadingGetUsersTrashed: true, errorGetUsersTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await UserTrashedCommand.findAllUsersTrashed(
          token,
          req,
        );

        console.log("response", response);

        set({
          users: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetUsersTrashed: false,
          errorGetUsersTrashed: null,
        });
      } else {
        const response = await UserTrashedService.findAllUsersTrashed(
          req,
          token,
        );
        console.log("response", response);

        set({
          users: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetUsersTrashed: false,
          errorGetUsersTrashed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetUsersTrashed: false }),
        (message: any) => set({ errorGetUsersTrashed: message }),
        req.toast,
      );
    }
  },

  restoreUserTrashed: async (req: RestoreUserTrashed) => {
    set({ loadingRestoreUserTrashed: true, errorRestoreUserTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await UserTrashedCommand.restoreUserTrashed(token, req);

        handleMessageAction("user", "restore");

        set({
          loadingRestoreUserTrashed: false,
          errorRestoreUserTrashed: null,
        });
      } else {
        await UserTrashedService.restoreUserTrashed(req, token);

        handleMessageAction("user", "restore");

        set({
          loadingRestoreUserTrashed: false,
          errorRestoreUserTrashed: null,
        });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreUserTrashed: false }),
        (message: any) => set({ errorRestoreUserTrashed: message }),

        req.toast,
      );
      return false;
    }
  },

  deletePermanentUser: async (req: DeletePermanentUser) => {
    set({
      loadingDeletePermanentUserTrashed: true,
      errorDeletePermanentUserTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await UserTrashedCommand.deletePermanentUser(token, req);
        set({
          loadingDeletePermanentUserTrashed: false,
          errorDeletePermanentUserTrashed: null,
        });

        handleMessageAction("user", "deletePermanent");
      } else {
        await UserTrashedService.deletePermanentUser(req, token);
        set({
          loadingDeletePermanentUserTrashed: false,
          errorDeletePermanentUserTrashed: null,
        });

        handleMessageAction("user", "deletePermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentUserTrashed: false }),
        (message: any) => set({ errorDeletePermanentUserTrashed: message }),
        req.toast,
      );

      return false;
    }
  },
  restoreUserAllTrashed: async (toast: any) => {
    set({
      loadingRestoreAllUserTrashed: true,
      errorRestoreAllUserTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await UserTrashedCommand.restoreUserAllTrashed(token);

        set({
          loadingRestoreAllUserTrashed: false,
          errorRestoreAllUserTrashed: null,
        });

        handleMessageAction("user", "restoreAll");
      } else {
        await UserTrashedService.restoreUserAllTrashed(token);

        set({
          loadingRestoreAllUserTrashed: false,
          errorRestoreAllUserTrashed: null,
        });

        handleMessageAction("user", "restoreAll");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreAllUserTrashed: false }),
        (message: any) => set({ errorRestoreAllUserTrashed: message }),
        toast,
      );

      return false;
    }
  },

  deletePermanentAllUser: async (toast: any) => {
    set({
      loadingDeletePermanentAllUserTrashed: true,
      errorDeletePermanentAllUserTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await UserTrashedCommand.deletePermanentAllUser(token);

        set({
          loadingDeletePermanentUserTrashed: false,
          errorDeletePermanentUserTrashed: null,
        });
        handleMessageAction("user", "deleteAllPermanent");
      } else {
        await UserTrashedService.deletePermanentAllUser(token);

        set({
          loadingDeletePermanentUserTrashed: false,
          errorDeletePermanentUserTrashed: null,
        });
        handleMessageAction("user", "deleteAllPermanent");
      }
      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentUserTrashed: false }),
        (message: any) => set({ errorDeletePermanentUserTrashed: message }),
        toast,
      );
      return false;
    }
  },
}));

export default useUserTrashedStore;
