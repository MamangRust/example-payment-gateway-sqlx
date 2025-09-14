import { create } from "zustand";
import { handleApiError } from "@/helpers/handleApi";
import { UserStore } from "@/types/state/user";
import {
  CreateUser,
  FindAllUser,
  FindByIdUser,
  TrashedUser,
  UpdateUser,
} from "@/types/domain/request";
import { getAccessToken } from "../auth";
import UserService from "@/services/api/user/user";
import UserCommand from "@/services/ipc/user/user";

import { handleMessageAction } from "@/helpers/message";
import { isTauri } from "@tauri-apps/api/core";

const useUserStore = create<UserStore>((set, _get) => ({
  users: null,
  user: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingGetUsers: false,
  loadingGetUser: false,
  loadingGetActiveUsers: false,
  loadingGetTrashedUsers: false,
  loadingCreateUser: false,
  loadingUpdateUser: false,
  loadingTrashedUser: false,

  errorGetUsers: null,
  errorGetUser: null,
  errorGetActiveUsers: null,
  errorGetTrashedUsers: null,
  errorCreateUser: null,
  errorUpdateUser: null,
  errorTrashedUser: null,

  setErrorGetUsers: (value: string | null) => set({ errorGetUsers: value }),
  setErrorGetUser: (value: string | null) => set({ errorGetUser: value }),
  setErrorGetActiveUsers: (value: string | null) =>
    set({ errorGetActiveUsers: value }),
  setErrorGetTrashedUsers: (value: string | null) =>
    set({ errorGetTrashedUsers: value }),
  setErrorCreateUser: (value: string | null) => set({ errorCreateUser: value }),
  setErrorUpdateUser: (value: string | null) => set({ errorUpdateUser: value }),
  setErrorTrashedUser: (value: string | null) =>
    set({ errorTrashedUser: value }),

  setLoadingGetUsers: (value: boolean) => set({ loadingGetUsers: value }),
  setLoadingGetUser: (value: boolean) => set({ loadingGetUser: value }),
  setLoadingGetActiveUsers: (value: boolean) =>
    set({ loadingGetActiveUsers: value }),
  setLoadingGetTrashedUsers: (value: boolean) =>
    set({ loadingGetTrashedUsers: value }),
  setLoadingCreateUser: (value: boolean) => set({ loadingCreateUser: value }),
  setLoadingUpdateUser: (value: boolean) => set({ loadingUpdateUser: value }),
  setLoadingTrashedUser: (value: boolean) => set({ loadingTrashedUser: value }),

  findAllUsers: async (req: FindAllUser) => {
    set({ loadingGetUsers: true, errorGetUsers: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await UserCommand.findAllUsers(token, req);

        set({
          users: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetUsers: false,
          errorGetUsers: null,
        });
      } else {
        const response = await UserService.findAllUsers(token, req);

        set({
          users: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetUsers: false,
          errorGetUsers: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetUsers: false }),
        (message: any) => set({ errorGetUsers: message }),
        req.toast,
      );
    }
  },

  findById: async (req: FindByIdUser) => {
    set({ loadingGetUser: true, errorGetUser: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await UserCommand.findByIdUser(token, req);

        set({
          user: response.data,
          loadingGetUser: false,
          errorGetUser: null,
        });
      } else {
        const response = await UserService.findById(token, req);

        set({
          user: response.data,
          loadingGetUser: false,
          errorGetUser: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetUser: false }),
        (message: any) => set({ errorGetUser: message }),
        req.toast,
      );
    }
  },

  findByActive: async (req: FindAllUser) => {
    set({ loadingGetActiveUsers: true, errorGetActiveUsers: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await UserCommand.findByActive(token, req);

        set({
          users: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetUsers: false,
          errorGetUsers: null,
        });
      } else {
        const response = await UserService.findByActive(token, req);

        set({
          users: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetUsers: false,
          errorGetUsers: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetActiveUsers: false }),
        (message: any) => set({ errorGetActiveUsers: message }),
        null,
      );
    }
  },

  createUser: async (req: CreateUser) => {
    set({ loadingCreateUser: true, errorCreateUser: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await UserCommand.createUser(token, req);

        handleMessageAction("", "");
      } else {
        await UserService.createUser(token, req);

        handleMessageAction("", "");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingCreateUser: false }),
        (message: any) => set({ errorCreateUser: message }),
        req.toast,
      );

      return false;
    }
  },

  updateUser: async (req: UpdateUser) => {
    set({ loadingUpdateUser: true, errorUpdateUser: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await UserCommand.updateUser(token, req);
      } else {
        await UserService.updateUser(token, req);
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingUpdateUser: false }),
        (message: any) => set({ errorUpdateUser: message }),
        req.toast,
      );

      return false;
    }
  },

  trashedUser: async (req: TrashedUser) => {
    set({ loadingTrashedUser: true, errorTrashedUser: null });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await UserCommand.trashedUser(token, req);
      } else {
        await UserService.trashedUser(token, req);
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingTrashedUser: false }),
        (message: any) => set({ errorTrashedUser: message }),
        req.toast,
      );
      return false;
    }
  },
}));

export default useUserStore;
