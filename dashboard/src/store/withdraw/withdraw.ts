import { WithdrawStore } from "@/types/state/withdraw/withdraw";
import { create } from "zustand";
import { getAccessToken } from "../auth";
import { handleApiError } from "@/helpers/handleApi";
import { FindAllWithdraw } from "@/types/domain/request/withdraw/list";
import {
  CreateWithdraw,
  FindByCardNumberWithdraw,
  FindByIdWithdraw,
  TrashedWithdraw,
  UpdateWithdraw,
} from "@/types/domain/request";
import WithdrawService from "@/services/api/withdraw/withdraw";
import WithdrawCommand from "@/services/ipc/withdraw/withdraw";
import { handleMessageAction } from "@/helpers/message";
import { isTauri } from "@tauri-apps/api/core";

const useWithdrawStore = create<WithdrawStore>((set, _get) => ({
  withdraws: null,
  withdraw: null,

  monthStatusSuccess: null,
  yearStatusSuccess: null,

  monthStatusFailed: null,
  yearStatusFailed: null,

  monthWithdrawAmount: null,
  yearWithdrawAmount: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingMonthStatusSuccess: false,
  loadingYearStatusSuccess: false,
  loadingMonthStatusFailed: false,
  loadingYearStatusFailed: false,
  loadingMonthWithdrawAmount: false,
  loadingYearWithdrawAmount: false,

  loadingGetWithdraws: false,
  loadingGetWithdraw: false,
  loadingGetCardNumberWithdraw: false,
  loadingGetActiveWithdraw: false,
  loadingGetTrashedWithdraw: false,

  loadingCreateWithdraw: false,
  loadingUpdateWithdraw: false,
  loadingTrashedWithdraw: false,
  loadingRestoreWithdraw: false,
  loadingPermanentWithdraw: false,

  errorMonthStatusSuccess: null,
  errorYearStatusSuccess: null,
  errorMonthStatusFailed: null,
  errorYearStatusFailed: null,
  errorMonthWithdrawAmount: null,
  errorYearWithdrawAmount: null,

  errorGetWithdraws: null,
  errorGetWithdraw: null,
  errorGetCardNumberWithdraw: null,
  errorGetActiveWithdraw: null,
  errorGetTrashedWithdraw: null,

  errorCreateWithdraw: null,
  errorUpdateWithdraw: null,
  errorTrashedWithdraw: null,
  errorRestoreWithdraw: null,
  errorPermanentWithdraw: null,

  setLoadingMonthStatusSuccess: (value) =>
    set({ loadingMonthStatusSuccess: value }),
  setLoadingYearStatusSuccess: (value) =>
    set({ loadingYearStatusSuccess: value }),
  setLoadingMonthStatusFailed: (value) =>
    set({ loadingMonthStatusFailed: value }),
  setLoadingYearStatusFailed: (value) =>
    set({ loadingYearStatusFailed: value }),
  setLoadingMonthWithdrawAmount: (value) =>
    set({ loadingMonthWithdrawAmount: value }),
  setLoadingYearWithdrawAmount: (value) =>
    set({ loadingYearWithdrawAmount: value }),

  setLoadingGetWithdraws: (value) => set({ loadingGetWithdraws: value }),
  setLoadingGetWithdraw: (value) => set({ loadingGetWithdraw: value }),
  setLoadingGetCardNumberWithdraw: (value) =>
    set({ loadingGetCardNumberWithdraw: value }),
  setLoadingGetActiveWithdraw: (value) =>
    set({ loadingGetActiveWithdraw: value }),
  setLoadingGetTrashedWithdraw: (value) =>
    set({ loadingGetTrashedWithdraw: value }),

  setLoadingCreateWithdraw: (value) => set({ loadingCreateWithdraw: value }),
  setLoadingUpdateWithdraw: (value) => set({ loadingUpdateWithdraw: value }),
  setLoadingTrashedWithdraw: (value) => set({ loadingTrashedWithdraw: value }),

  setErrorMonthStatusSuccess: (value) =>
    set({ errorMonthStatusSuccess: value }),
  setErrorYearStatusSuccess: (value) => set({ errorYearStatusSuccess: value }),
  setErrorMonthStatusFailed: (value) => set({ errorMonthStatusFailed: value }),
  setErrorYearStatusFailed: (value) => set({ errorYearStatusFailed: value }),
  setErrorMonthWithdrawAmount: (value) =>
    set({ errorMonthWithdrawAmount: value }),
  setErrorYearWithdrawAmount: (value) =>
    set({ errorYearWithdrawAmount: value }),

  setErrorGetWithdraws: (value) => set({ errorGetWithdraws: value }),
  setErrorGetWithdraw: (value) => set({ errorGetWithdraw: value }),
  setErrorGetCardNumberWithdraw: (value) =>
    set({ errorGetCardNumberWithdraw: value }),
  setErrorGetActiveWithdraw: (value) => set({ errorGetActiveWithdraw: value }),
  setErrorGetTrashedWithdraw: (value) =>
    set({ errorGetTrashedWithdraw: value }),

  setErrorCreateWithdraw: (value) => set({ errorCreateWithdraw: value }),
  setErrorUpdateWithdraw: (value) => set({ errorUpdateWithdraw: value }),
  setErrorTrashedWithdraw: (value) => set({ errorTrashedWithdraw: value }),

  findMonthStatusSuccess: async (toast: any, year: number, month: number) => {
    set({ loadingMonthStatusSuccess: true, errorMonthStatusSuccess: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findMonthStatusSuccessWithdraw(
          token,
          year,
          month,
        );

        set({
          monthStatusSuccess: response.data,
          loadingMonthStatusSuccess: false,
          errorMonthStatusSuccess: null,
        });
      } else {
        const response = await WithdrawService.findMonthStatusSuccess(
          token,
          year,
          month,
        );

        set({
          monthStatusSuccess: response,
          loadingMonthStatusSuccess: false,
          errorMonthStatusSuccess: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthStatusSuccess: false }),
        (message: any) => set({ errorMonthStatusSuccess: message }),
        toast,
      );
    }
  },

  findYearStatusSuccess: async (toast: any, year: number) => {
    set({ loadingYearStatusSuccess: true, errorYearStatusSuccess: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findYearStatusSuccessWithdraw(
          token,
          year,
        );

        set({
          yearStatusSuccess: response.data,
          loadingYearStatusSuccess: false,
          errorYearStatusSuccess: null,
        });
      } else {
        const response = await WithdrawService.findYearStatusSuccess(
          token,
          year,
        );

        set({
          yearStatusSuccess: response,
          loadingYearStatusSuccess: false,
          errorYearStatusSuccess: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearStatusSuccess: false }),
        (message: any) => set({ errorYearStatusSuccess: message }),
        toast,
      );
    }
  },

  findMonthStatusFailed: async (toast: any, year: number, month: number) => {
    set({ loadingMonthStatusFailed: true, errorMonthStatusFailed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findMonthStatusFailedWithdraw(
          token,
          year,
          month,
        );

        set({
          monthStatusFailed: response.data,
          loadingMonthStatusFailed: false,
          errorMonthStatusFailed: null,
        });
      } else {
        const response = await WithdrawService.findMonthStatusFailed(
          token,
          year,
          month,
        );

        set({
          monthStatusFailed: response,
          loadingMonthStatusFailed: false,
          errorMonthStatusFailed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthStatusFailed: false }),
        (message: any) => set({ errorMonthStatusFailed: message }),
        toast,
      );
    }
  },

  findYearStatusFailed: async (toast: any, year: number) => {
    set({ loadingYearStatusFailed: true, errorYearStatusFailed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findYearStatusFailedWithdraw(
          token,
          year,
        );

        set({
          yearStatusFailed: response.data,
          loadingYearStatusFailed: false,
          errorYearStatusFailed: null,
        });
      } else {
        const response = await WithdrawService.findYearStatusFailed(
          token,
          year,
        );

        set({
          yearStatusFailed: response,
          loadingYearStatusFailed: false,
          errorYearStatusFailed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearStatusFailed: false }),
        (message: any) => set({ errorYearStatusFailed: message }),
        toast,
      );
    }
  },

  findMonthStatusSuccessByCardNumber: async (
    toast: any,
    year: number,
    month: number,
    cardNumber: string,
  ) => {
    set({ loadingMonthStatusSuccess: true, errorMonthStatusSuccess: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await WithdrawCommand.findMonthStatusSuccessWithdrawByCardNumber(
            token,
            year,
            month,
            cardNumber,
          );

        set({
          monthStatusSuccess: response.data,
          loadingMonthStatusSuccess: false,
          errorMonthStatusSuccess: null,
        });
      } else {
        const response =
          await WithdrawService.findMonthStatusSuccessByCardNumber(
            token,
            year,
            month,
            cardNumber,
          );

        set({
          monthStatusSuccess: response,
          loadingMonthStatusSuccess: false,
          errorMonthStatusSuccess: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthStatusSuccess: false }),
        (message: any) => set({ errorMonthStatusSuccess: message }),
        toast,
      );
    }
  },

  findYearStatusSuccessByCardNumber: async (
    toast: any,
    year: number,
    cardNumber: string,
  ) => {
    set({ loadingYearStatusSuccess: true, errorYearStatusSuccess: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await WithdrawCommand.findYearStatusSuccessWithdrawByCardNumber(
            token,
            year,
            cardNumber,
          );

        set({
          yearStatusSuccess: response.data,
          loadingYearStatusSuccess: false,
          errorYearStatusSuccess: null,
        });
      } else {
        const response =
          await WithdrawService.findYearStatusSuccessByCardNumber(
            token,
            year,
            cardNumber,
          );

        set({
          yearStatusSuccess: response,
          loadingYearStatusSuccess: false,
          errorYearStatusSuccess: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearStatusSuccess: false }),
        (message: any) => set({ errorYearStatusSuccess: message }),
        toast,
      );
    }
  },

  findMonthStatusFailedByCardNumber: async (
    toast: any,
    year: number,
    month: number,
    cardNumber: string,
  ) => {
    set({ loadingMonthStatusFailed: true, errorMonthStatusFailed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await WithdrawCommand.findMonthStatusFailedWithdrawByCardNumber(
            token,
            year,
            month,
            cardNumber,
          );

        set({
          monthStatusFailed: response.data,
          loadingMonthStatusFailed: false,
          errorMonthStatusFailed: null,
        });
      } else {
        const response =
          await WithdrawService.findMonthStatusFailedByCardNumber(
            token,
            year,
            month,
            cardNumber,
          );

        set({
          monthStatusFailed: response,
          loadingMonthStatusFailed: false,
          errorMonthStatusFailed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthStatusFailed: false }),
        (message: any) => set({ errorMonthStatusFailed: message }),
        toast,
      );
    }
  },

  findYearStatusFailedByCardNumber: async (
    toast: any,
    year: number,
    cardNumber: string,
  ) => {
    set({ loadingYearStatusFailed: true, errorYearStatusFailed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await WithdrawCommand.findYearStatusFailedWithdrawByCardNumber(
            token,
            year,
            cardNumber,
          );

        set({
          yearStatusFailed: response.data,
          loadingYearStatusFailed: false,
          errorYearStatusFailed: null,
        });
      } else {
        const response = await WithdrawService.findYearStatusFailedByCardNumber(
          token,
          year,
          cardNumber,
        );

        set({
          yearStatusFailed: response,
          loadingYearStatusFailed: false,
          errorYearStatusFailed: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearStatusFailed: false }),
        (message: any) => set({ errorYearStatusFailed: message }),
        toast,
      );
    }
  },

  findMonthWithdrawAmount: async (toast: any, year: number) => {
    set({ loadingMonthWithdrawAmount: true, errorMonthWithdrawAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findMonthWithdrawAmountWithdraw(
          token,
          year,
        );

        set({
          monthWithdrawAmount: response.data,
          loadingMonthWithdrawAmount: false,
          errorMonthWithdrawAmount: null,
        });
      } else {
        const response = await WithdrawService.findMonthWithdrawAmount(
          token,
          year,
        );

        set({
          monthWithdrawAmount: response,
          loadingMonthWithdrawAmount: false,
          errorMonthWithdrawAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthWithdrawAmount: false }),
        (message: any) => set({ errorMonthWithdrawAmount: message }),
        toast,
      );
    }
  },

  findYearWithdrawAmount: async (toast: any, year: number) => {
    set({ loadingYearWithdrawAmount: true, errorYearWithdrawAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findYearWithdrawAmount(
          token,
          year,
        );

        set({
          yearWithdrawAmount: response.data,
          loadingYearWithdrawAmount: false,
          errorYearWithdrawAmount: null,
        });
      } else {
        const response = await WithdrawService.findYearWithdrawAmount(
          token,
          year,
        );

        set({
          yearWithdrawAmount: response,
          loadingYearWithdrawAmount: false,
          errorYearWithdrawAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearWithdrawAmount: false }),
        (message: any) => set({ errorYearWithdrawAmount: message }),
        toast,
      );
    }
  },

  findMonthWithdrawAmountByCard: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({ loadingMonthWithdrawAmount: true, errorMonthWithdrawAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await WithdrawCommand.findMonthWithdrawAmountByCardWithdraw(
            token,
            year,
            card_number,
          );
        set({
          monthWithdrawAmount: response.data,
          loadingMonthWithdrawAmount: false,
          errorMonthWithdrawAmount: null,
        });
      } else {
        const response = await WithdrawService.findMonthWithdrawAmountByCard(
          token,
          year,
          card_number,
        );
        set({
          monthWithdrawAmount: response,
          loadingMonthWithdrawAmount: false,
          errorMonthWithdrawAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthWithdrawAmount: false }),
        (message: any) => set({ errorMonthWithdrawAmount: message }),
        toast,
      );
    }
  },

  findYearWithdrawAmountByCard: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({ loadingYearWithdrawAmount: true, errorYearWithdrawAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findYearWithdrawAmountByCard(
          token,
          year,
          card_number,
        );

        set({
          yearWithdrawAmount: response.data,
          loadingYearWithdrawAmount: false,
          errorYearWithdrawAmount: null,
        });
      } else {
        const response = await WithdrawService.findYearWithdrawAmountByCard(
          token,
          year,
          card_number,
        );

        set({
          yearWithdrawAmount: response,
          loadingYearWithdrawAmount: false,
          errorYearWithdrawAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearWithdrawAmount: false }),
        (message: any) => set({ errorYearWithdrawAmount: message }),
        toast,
      );
    }
  },

  findAllWithdraws: async (req: FindAllWithdraw) => {
    set({ loadingGetWithdraws: true, errorGetWithdraws: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findAllWithdraws(token, req);

        set({
          withdraws: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetWithdraws: false,
          errorGetWithdraws: null,
        });
      } else {
        const response = await WithdrawService.findAllWithdraws(token, req);

        set({
          withdraws: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetWithdraws: false,
          errorGetWithdraws: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetWithdraws: false }),
        (message: any) => set({ errorGetWithdraws: message }),
        req.toast,
      );
    }
  },

  findByIdWithdraw: async (req: FindByIdWithdraw) => {
    set({ loadingGetWithdraw: true, errorGetWithdraw: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findByIdWithdraw(token, req);

        set({
          withdraw: response.data,
          loadingGetWithdraw: false,
          errorGetWithdraw: null,
        });
      } else {
        const response = await WithdrawService.findByIdWithdraw(token, req);

        set({
          withdraw: response,
          loadingGetWithdraw: false,
          errorGetWithdraw: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetWithdraw: false }),
        (message: any) => set({ errorGetWithdraw: message }),
        req.toast,
      );
    }
  },

  findByCardNumberWithdraw: async (req: FindByCardNumberWithdraw) => {
    set({
      loadingGetCardNumberWithdraw: true,
      errorGetCardNumberWithdraw: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findByCardNumberWithdraw(
          token,
          req.cardNumber,
        );

        set({
          withdraws: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetCardNumberWithdraw: false,
          errorGetCardNumberWithdraw: null,
        });
      } else {
        const response = await WithdrawService.findAllByCardNumberWithdraw(
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
          loadingGetCardNumberWithdraw: false,
          errorGetCardNumberWithdraw: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetCardNumberWithdraw: false }),
        (message: any) => set({ errorGetCardNumberWithdraw: message }),
        req.toast,
      );
    }
  },

  findByActiveWithdraw: async (req: FindAllWithdraw) => {
    set({ loadingGetActiveWithdraw: true, errorGetActiveWithdraw: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await WithdrawCommand.findByActiveWithdraw(token, req);

        set({
          withdraws: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },

          loadingGetActiveWithdraw: false,
          errorGetActiveWithdraw: null,
        });
      } else {
        const response = await WithdrawService.findByActiveWithdraw(token, req);

        set({
          withdraws: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },

          loadingGetActiveWithdraw: false,
          errorGetActiveWithdraw: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetActiveWithdraw: false }),
        (message: any) => set({ errorGetActiveWithdraw: message }),
        null,
      );
    }
  },

  createWithdraw: async (req: CreateWithdraw) => {
    set({ loadingCreateWithdraw: true, errorCreateWithdraw: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await WithdrawCommand.createWithdraw(token, req);

        handleMessageAction("withdraw", "create");

        set({ loadingCreateWithdraw: false, errorCreateWithdraw: null });
      } else {
        await WithdrawService.createWithdraw(token, req);

        handleMessageAction("withdraw", "create");

        set({ loadingCreateWithdraw: false, errorCreateWithdraw: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingCreateWithdraw: false }),
        (message: any) => set({ errorCreateWithdraw: message }),
        req.toast,
      );

      return false;
    }
  },

  updateWithdraw: async (req: UpdateWithdraw) => {
    set({ loadingUpdateWithdraw: true, errorUpdateWithdraw: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await WithdrawCommand.updateWithdraw(token, req);

        handleMessageAction("withdraw", "update");

        set({ loadingUpdateWithdraw: false, errorUpdateWithdraw: null });
      } else {
        await WithdrawService.updateWithdraw(token, req);

        handleMessageAction("withdraw", "update");

        set({ loadingUpdateWithdraw: false, errorUpdateWithdraw: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingUpdateWithdraw: false }),
        (message: any) => set({ errorUpdateWithdraw: message }),
        req.toast,
      );
      return false;
    }
  },

  trashedWithdraw: async (req: TrashedWithdraw) => {
    set({ loadingTrashedWithdraw: true, errorTrashedWithdraw: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await WithdrawCommand.trashedWithdraw(token, req);

        handleMessageAction("withdraw", "restore");
        set({ loadingTrashedWithdraw: false, errorTrashedWithdraw: null });
      } else {
        await WithdrawService.trashedWithdraw(token, req);

        handleMessageAction("withdraw", "restore");
        set({ loadingTrashedWithdraw: false, errorTrashedWithdraw: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingTrashedWithdraw: false }),
        (message: any) => set({ errorTrashedWithdraw: message }),
        req.id,
      );
      return false;
    }
  },
}));

export default useWithdrawStore;
