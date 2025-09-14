import { handleApiError } from "@/helpers/handleApi";
import { handleMessageAction } from "@/helpers/message";
import { getAccessToken } from "@/store/auth";
import {
  DeletePermanentRole,
  FindAllRoleTrashed,
  RestoreRoleTrashed,
} from "@/types/domain/request";
import { RoleTrashedStore } from "@/types/state";
import { create } from "zustand";
import RoleTrashedCommand from "@/services/ipc/role/role_trashed";
import RoleTrashedService from "@/services/api/role/role_trashed";
import { isTauri } from "@tauri-apps/api/core";

const useRoleTrashedStore = create<RoleTrashedStore>((set, _get) => ({
  Roles: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingGetRolesTrashed: false,
  loadingRestoreRoleTrashed: false,
  loadingDeletePermanentRoleTrashed: false,
  loadingRestoreAllRoleTrashed: false,
  loadingDeletePermanentAllRoleTrashed: false,

  errorGetRolesTrashed: false,
  errorRestoreRoleTrashed: false,
  errorDeletePermanentRoleTrashed: false,
  errorRestoreAllRoleTrashed: false,
  errorDeletePermanentAllRoleTrashed: false,

  setLoadingGetRolesTrashed: (value: boolean) =>
    set({ loadingGetRolesTrashed: value }),
  setLoadingRestoreRoleTrashed: (value: boolean) =>
    set({ loadingRestoreRoleTrashed: value }),
  setLoadingDeletePermanentRoleTrashed: (value: boolean) =>
    set({ loadingDeletePermanentRoleTrashed: value }),
  setLoadingRestoreAllRoleTrashed: (value: boolean) =>
    set({ loadingRestoreAllRoleTrashed: value }),
  setLoadingDeletePermanentAllRoleTrashed: (value: boolean) =>
    set({ loadingDeletePermanentAllRoleTrashed: value }),

  setErrorGetRolesTrashed: (value: boolean) =>
    set({ errorGetRolesTrashed: value }),
  setErrorRestoreRoleTrashed: (value: boolean) =>
    set({ errorRestoreRoleTrashed: value }),
  setErrorDeletePermanentRoleTrashed: (value: boolean) =>
    set({ errorDeletePermanentRoleTrashed: value }),
  setErrorRestoreAllRoleTrashed: (value: boolean) =>
    set({ errorRestoreAllRoleTrashed: value }),
  setErrorDeletePermanentAllRoleTrashed: (value: boolean) =>
    set({ errorDeletePermanentAllRoleTrashed: value }),

  findAllRolesTrashed: async (req: FindAllRoleTrashed) => {
    set({ loadingGetRolesTrashed: true, errorGetRolesTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await RoleTrashedCommand.findAllRolesTrashed(
          token,
          req,
        );

        console.log("response", response);

        set({
          Roles: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetRolesTrashed: false,
          errorGetRolesTrashed: null,
        });
      } else {
        const response = await RoleTrashedService.findAllRolesTrashed(
          req,
          token,
        );
        console.log("response", response);

        set({
          Roles: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetRolesTrashed: false,
          errorGetRolesTrashed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetRolesTrashed: false }),
        (message: any) => set({ errorGetRolesTrashed: message }),
        req.toast,
      );
    }
  },

  restoreRoleTrashed: async (req: RestoreRoleTrashed) => {
    set({ loadingRestoreRoleTrashed: true, errorRestoreRoleTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await RoleTrashedCommand.restoreRoleTrashed(token, req);

        handleMessageAction("Role", "restore");

        set({
          loadingRestoreRoleTrashed: false,
          errorRestoreRoleTrashed: null,
        });
      } else {
        await RoleTrashedService.restoreRoleTrashed(req, token);

        handleMessageAction("Role", "restore");

        set({
          loadingRestoreRoleTrashed: false,
          errorRestoreRoleTrashed: null,
        });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreRoleTrashed: false }),
        (message: any) => set({ errorRestoreRoleTrashed: message }),

        req.toast,
      );
      return false;
    }
  },

  deletePermanentRole: async (req: DeletePermanentRole) => {
    set({
      loadingDeletePermanentRoleTrashed: true,
      errorDeletePermanentRoleTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await RoleTrashedCommand.deletePermanentRole(token, req);
        set({
          loadingDeletePermanentRoleTrashed: false,
          errorDeletePermanentRoleTrashed: null,
        });

        handleMessageAction("Role", "deletePermanent");
      } else {
        await RoleTrashedService.deletePermanentRole(req, token);
        set({
          loadingDeletePermanentRoleTrashed: false,
          errorDeletePermanentRoleTrashed: null,
        });

        handleMessageAction("Role", "deletePermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentRoleTrashed: false }),
        (message: any) => set({ errorDeletePermanentRoleTrashed: message }),
        req.toast,
      );

      return false;
    }
  },
  restoreRoleAllTrashed: async (toast: any) => {
    set({
      loadingRestoreAllRoleTrashed: true,
      errorRestoreAllRoleTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await RoleTrashedCommand.restoreRoleAllTrashed(token);

        set({
          loadingRestoreAllRoleTrashed: false,
          errorRestoreAllRoleTrashed: null,
        });

        handleMessageAction("Role", "restoreAll");
      } else {
        await RoleTrashedService.restoreRoleAllTrashed(token);

        set({
          loadingRestoreAllRoleTrashed: false,
          errorRestoreAllRoleTrashed: null,
        });

        handleMessageAction("Role", "restoreAll");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreAllRoleTrashed: false }),
        (message: any) => set({ errorRestoreAllRoleTrashed: message }),
        toast,
      );

      return false;
    }
  },

  deletePermanentAllRole: async (toast: any) => {
    set({
      loadingDeletePermanentAllRoleTrashed: true,
      errorDeletePermanentAllRoleTrashed: false,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await RoleTrashedCommand.deletePermanentAllRole(token);

        set({
          loadingDeletePermanentRoleTrashed: false,
          errorDeletePermanentRoleTrashed: null,
        });
        handleMessageAction("Role", "deleteAllPermanent");
      } else {
        await RoleTrashedService.deletePermanentAllRole(token);

        set({
          loadingDeletePermanentRoleTrashed: false,
          errorDeletePermanentRoleTrashed: null,
        });
        handleMessageAction("Role", "deleteAllPermanent");
      }
      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentRoleTrashed: false }),
        (message: any) => set({ errorDeletePermanentRoleTrashed: message }),
        toast,
      );
      return false;
    }
  },
}));

export default useRoleTrashedStore;
