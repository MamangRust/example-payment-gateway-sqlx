import { create } from "zustand";
import { handleApiError } from "@/helpers/handleApi";
import { RoleStore } from "@/types/state/role";
import {
  CreateRole,
  FindAllRole,
  FindByIdRole,
  TrashedRole,
  UpdateRole,
} from "@/types/domain/request";
import { getAccessToken } from "../auth";
import RoleService from "@/services/api/role/role";
import RoleCommand from "@/services/ipc/role/role";
import { FindAllRoleTrashed } from "@/types/domain/request/role/trashed/list";

import { handleMessageAction } from "@/helpers/message";
import { isTauri } from "@tauri-apps/api/core";

const useRoleStore = create<RoleStore>((set, _get) => ({
  Roles: null,
  Role: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingGetRoles: false,
  loadingGetRole: false,
  loadingGetActiveRoles: false,
  loadingGetTrashedRoles: false,
  loadingCreateRole: false,
  loadingUpdateRole: false,
  loadingTrashedRole: false,

  errorGetRoles: null,
  errorGetRole: null,
  errorGetActiveRoles: null,
  errorGetTrashedRoles: null,
  errorCreateRole: null,
  errorUpdateRole: null,
  errorTrashedRole: null,

  setErrorGetRoles: (value: string | null) => set({ errorGetRoles: value }),
  setErrorGetRole: (value: string | null) => set({ errorGetRole: value }),
  setErrorGetActiveRoles: (value: string | null) =>
    set({ errorGetActiveRoles: value }),
  setErrorGetTrashedRoles: (value: string | null) =>
    set({ errorGetTrashedRoles: value }),
  setErrorCreateRole: (value: string | null) => set({ errorCreateRole: value }),
  setErrorUpdateRole: (value: string | null) => set({ errorUpdateRole: value }),
  setErrorTrashedRole: (value: string | null) =>
    set({ errorTrashedRole: value }),

  setLoadingGetRoles: (value: boolean) => set({ loadingGetRoles: value }),
  setLoadingGetRole: (value: boolean) => set({ loadingGetRole: value }),
  setLoadingGetActiveRoles: (value: boolean) =>
    set({ loadingGetActiveRoles: value }),
  setLoadingGetTrashedRoles: (value: boolean) =>
    set({ loadingGetTrashedRoles: value }),
  setLoadingCreateRole: (value: boolean) => set({ loadingCreateRole: value }),
  setLoadingUpdateRole: (value: boolean) => set({ loadingUpdateRole: value }),
  setLoadingTrashedRole: (value: boolean) => set({ loadingTrashedRole: value }),

  findAllRoles: async (req: FindAllRoleTrashed) => {
    set({ loadingGetRoles: true, errorGetRoles: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await RoleCommand.findAllRoles(token, req);

        set({
          Roles: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetRoles: false,
          errorGetRoles: null,
        });
      } else {
        const response = await RoleService.findAllRoles(token, req);

        set({
          Roles: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetRoles: false,
          errorGetRoles: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetRoles: false }),
        (message: any) => set({ errorGetRoles: message }),
        req.toast,
      );
    }
  },

  findById: async (req: FindByIdRole) => {
    set({ loadingGetRole: true, errorGetRole: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await RoleCommand.findByIdRole(token, req);

        set({
          Role: response.data,
          loadingGetRole: false,
          errorGetRole: null,
        });
      } else {
        const response = await RoleService.findById(token, req);

        set({
          Role: response.data,
          loadingGetRole: false,
          errorGetRole: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetRole: false }),
        (message: any) => set({ errorGetRole: message }),
        req.toast,
      );
    }
  },

  findByActive: async (req: FindAllRole) => {
    set({ loadingGetActiveRoles: true, errorGetActiveRoles: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await RoleCommand.findByActive(token, req);

        set({
          Roles: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetRoles: false,
          errorGetRoles: null,
        });
      } else {
        const response = await RoleService.findByActive(token, req);

        set({
          Roles: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetRoles: false,
          errorGetRoles: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetActiveRoles: false }),
        (message: any) => set({ errorGetActiveRoles: message }),
        null,
      );
    }
  },

  createRole: async (req: CreateRole) => {
    set({ loadingCreateRole: true, errorCreateRole: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await RoleCommand.createRole(token, req);

        handleMessageAction("", "");
      } else {
        await RoleService.createRole(token, req);

        handleMessageAction("", "");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingCreateRole: false }),
        (message: any) => set({ errorCreateRole: message }),
        req.toast,
      );

      return false;
    }
  },

  updateRole: async (req: UpdateRole) => {
    set({ loadingUpdateRole: true, errorUpdateRole: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await RoleCommand.updateRole(token, req);
      } else {
        await RoleService.updateRole(token, req);
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingUpdateRole: false }),
        (message: any) => set({ errorUpdateRole: message }),
        req.toast,
      );

      return false;
    }
  },

  trashedRole: async (req: TrashedRole) => {
    set({ loadingTrashedRole: true, errorTrashedRole: null });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await RoleCommand.trashedRole(token, req);
      } else {
        await RoleService.trashedRole(token, req);
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingTrashedRole: false }),
        (message: any) => set({ errorTrashedRole: message }),
        req.toast,
      );
      return false;
    }
  },
}));

export default useRoleStore;
