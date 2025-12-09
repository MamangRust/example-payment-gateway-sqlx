import { TopupStore } from "@/types/state/topup/topup";
import { create } from "zustand";
import { getAccessToken } from "../auth";
import { handleApiError } from "@/helpers/handleApi";
import {
  CreateTopup,
  FindAllTopup,
  FindByIdTopup,
  TrashedTopup,
  UpdateTopup,
} from "@/types/domain/request/topup";
import TopupService from "@/services/api/topup/topup";
import TopupCommand from "@/services/ipc/topup/topup";
import { FindByCardNumberTopup } from "@/types/domain/request/topup/findByCardNumber";
import { handleMessageAction } from "@/helpers/message";
import { isTauri } from "@tauri-apps/api/core";

const useTopupStore = create<TopupStore>((set, _get) => ({
  topups: null,
  topup: null,

  monthStatusSuccess: null,
  yearStatusSuccess: null,

  monthStatusFailed: null,
  yearStatusFailed: null,

  monthTopupMethod: null,
  yearTopupMethod: null,

  monthTopupAmount: null,
  yearTopupAmount: null,

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
  loadingMonthTopupMethod: false,
  loadingYearTopupMethod: false,
  loadingMonthTopupAmount: false,
  loadingYearTopupAmount: false,

  loadingGetTopups: false,
  loadingGetTopup: false,
  loadingGetActiveTopup: false,
  loadingGetTrashedTopup: false,
  loadingGetCardNumberTopup: false,

  loadingCreateTopup: false,
  loadingUpdateTopup: false,
  loadingTrashedTopup: false,
  loadingRestoreTopup: false,
  loadingPermanentTopup: false,

  errorMonthStatusSuccess: null,
  errorYearStatusSuccess: null,
  errorMonthStatusFailed: null,
  errorYearStatusFailed: null,
  errorMonthTopupMethod: null,
  errorYearTopupMethod: null,
  errorMonthTopupAmount: null,
  errorYearTopupAmount: null,

  errorGetTopups: null,
  errorGetTopup: null,
  errorGetActiveTopup: null,
  errorGetTrashedTopup: null,
  errorGetCardNumberTopup: null,

  errorCreateTopup: null,
  errorUpdateTopup: null,
  errorTrashedTopup: null,
  errorRestoreTopup: null,
  errorPermanentTopup: null,

  setLoadingMonthStatusSuccess: (value) =>
    set({ loadingMonthStatusSuccess: value }),
  setLoadingYearStatusSuccess: (value) =>
    set({ loadingYearStatusSuccess: value }),
  setLoadingMonthStatusFailed: (value) =>
    set({ loadingMonthStatusFailed: value }),
  setLoadingYearStatusFailed: (value) =>
    set({ loadingYearStatusFailed: value }),
  setLoadingMonthTopupMethod: (value) =>
    set({ loadingMonthTopupMethod: value }),
  setLoadingYearTopupMethod: (value) => set({ loadingYearTopupMethod: value }),
  setLoadingMonthTopupAmount: (value) =>
    set({ loadingMonthTopupAmount: value }),
  setLoadingYearTopupAmount: (value) => set({ loadingYearTopupAmount: value }),

  setLoadingGetTopups: (value) => set({ loadingGetTopups: value }),
  setLoadingGetTopup: (value) => set({ loadingGetTopup: value }),
  setLoadingGetActiveTopup: (value) => set({ loadingGetActiveTopup: value }),
  setLoadingGetTrashedTopup: (value) => set({ loadingGetTrashedTopup: value }),
  setLoadingGetCardNumberTopup: (value) =>
    set({ loadingGetCardNumberTopup: value }),

  setLoadingCreateTopup: (value) => set({ loadingCreateTopup: value }),
  setLoadingUpdateTopup: (value) => set({ loadingUpdateTopup: value }),
  setLoadingTrashedTopup: (value) => set({ loadingTrashedTopup: value }),

  setErrorMonthStatusSuccess: (value) =>
    set({ errorMonthStatusSuccess: value }),
  setErrorYearStatusSuccess: (value) => set({ errorYearStatusSuccess: value }),
  setErrorMonthStatusFailed: (value) => set({ errorMonthStatusFailed: value }),
  setErrorYearStatusFailed: (value) => set({ errorYearStatusFailed: value }),
  setErrorMonthTopupMethod: (value) => set({ errorMonthTopupMethod: value }),
  setErrorYearTopupMethod: (value) => set({ errorYearTopupMethod: value }),
  setErrorMonthTopupAmount: (value) => set({ errorMonthTopupAmount: value }),
  setErrorYearTopupAmount: (value) => set({ errorYearTopupAmount: value }),

  setErrorGetTopups: (value) => set({ errorGetTopups: value }),
  setErrorGetTopup: (value) => set({ errorGetTopup: value }),
  setErrorGetActiveTopup: (value) => set({ errorGetActiveTopup: value }),
  setErrorGetTrashedTopup: (value) => set({ errorGetTrashedTopup: value }),
  setErrorGetCardNumberTopup: (value) =>
    set({ errorGetCardNumberTopup: value }),

  setErrorCreateTopup: (value) => set({ errorCreateTopup: value }),
  setErrorUpdateTopup: (value) => set({ errorUpdateTopup: value }),
  setErrorTrashedTopup: (value) => set({ errorTrashedTopup: value }),

  findMonthStatusSuccess: async (toast: any, year: number, month: number) => {
    set({ loadingMonthStatusSuccess: true, errorMonthStatusSuccess: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findMonthStatusSuccess(
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
        const response = await TopupService.findMonthStatusSuccess(
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
        const response = await TopupCommand.findYearStatusSuccess(token, year);

        set({
          yearStatusSuccess: response.data,
          loadingYearStatusSuccess: false,
          errorYearStatusSuccess: null,
        });
      } else {
        const response = await TopupService.findYearStatusSuccess(token, year);

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
        const response = await TopupCommand.findMonthStatusFailed(
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
        const response = await TopupService.findMonthStatusFailed(
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
        const response = await TopupCommand.findYearStatusFailed(token, year);

        set({
          yearStatusFailed: response.data,
          loadingYearStatusFailed: false,
          errorYearStatusFailed: null,
        });
      } else {
        const response = await TopupService.findYearStatusFailed(token, year);

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
        const response = await TopupCommand.findMonthStatusSuccessByCardNumber(
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
        const response = await TopupService.findMonthStatusSuccessByCardNumber(
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
        const response = await TopupCommand.findYearStatusSuccessByCardNumber(
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
        const response = await TopupService.findYearStatusSuccessByCardNumber(
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
        const response = await TopupCommand.findMonthStatusFailedByCardNumber(
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
        const response = await TopupService.findMonthStatusFailedByCardNumber(
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
        const response = await TopupCommand.findYearStatusFailedByCardNumber(
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
        const response = await TopupService.findYearStatusFailedByCardNumber(
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

  findMonthTopupMethod: async (toast: any, year: number) => {
    set({ loadingMonthTopupMethod: true, errorMonthTopupMethod: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findMonthTopupMethod(token, year);

        set({
          monthTopupMethod: response.data,
          loadingMonthTopupMethod: false,
          errorMonthTopupMethod: null,
        });
      } else {
        const response = await TopupService.findMonthTopupMethod(token, year);

        set({
          monthTopupMethod: response,
          loadingMonthTopupMethod: false,
          errorMonthTopupMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTopupMethod: false }),
        (message: any) => set({ errorMonthTopupMethod: message }),
        toast,
      );
    }
  },

  findYearTopupMethod: async (toast: any, year: number) => {
    set({ loadingYearTopupMethod: true, errorYearTopupMethod: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findYearTopupMethod(token, year);

        set({
          yearTopupMethod: response.data,
          loadingYearTopupMethod: false,
          errorYearTopupMethod: null,
        });
      } else {
        const response = await TopupService.findYearTopupMethod(token, year);

        set({
          yearTopupMethod: response,
          loadingYearTopupMethod: false,
          errorYearTopupMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTopupMethod: false }),
        (message: any) => set({ errorYearTopupMethod: message }),
        toast,
      );
    }
  },

  findMonthTopupAmount: async (toast, year: number) => {
    set({ loadingMonthTopupAmount: true, errorMonthTopupAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findMonthTopupAmount(token, year);

        set({
          monthTopupAmount: response.data,
          loadingMonthTopupAmount: false,
          errorMonthTopupAmount: null,
        });
      } else {
        const response = await TopupService.findMonthTopupAmount(token, year);

        set({
          monthTopupAmount: response,
          loadingMonthTopupAmount: false,
          errorMonthTopupAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTopupAmount: false }),
        (message: any) => set({ errorMonthTopupAmount: message }),
        toast,
      );
    }
  },

  findYearTopupAmount: async (toast: any, year: number) => {
    set({ loadingYearTopupAmount: true, errorYearTopupAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findYearTopupAmount(token, year);

        set({
          yearTopupAmount: response.data,
          loadingYearTopupAmount: false,
          errorYearTopupAmount: null,
        });
      } else {
        const response = await TopupService.findYearTopupAmount(token, year);

        set({
          yearTopupAmount: response,
          loadingYearTopupAmount: false,
          errorYearTopupAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTopupAmount: false }),
        (message: any) => set({ errorYearTopupAmount: message }),
        toast,
      );
    }
  },

  findMonthTopupMethodCard: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({ loadingMonthTopupMethod: true, errorMonthTopupMethod: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findMonthTopupMethodByCard(
          token,
          year,
          card_number,
        );

        set({
          monthTopupMethod: response.data,
          loadingMonthTopupMethod: false,
          errorMonthTopupMethod: null,
        });
      } else {
        const response = await TopupService.findMonthTopupMethodCard(
          token,
          year,
          card_number,
        );

        set({
          monthTopupMethod: response,
          loadingMonthTopupMethod: false,
          errorMonthTopupMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTopupMethod: false }),
        (message: any) => set({ errorMonthTopupMethod: message }),
        toast,
      );
    }
  },

  findYearTopupMethodCard: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({ loadingYearTopupMethod: true, errorYearTopupMethod: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findYearTopupMethodByCard(
          token,
          year,
          card_number,
        );

        set({
          yearTopupMethod: response.data,
          loadingYearTopupMethod: false,
          errorYearTopupMethod: null,
        });
      } else {
        const response = await TopupService.findYearTopupMethodCard(
          token,
          year,
          card_number,
        );

        set({
          yearTopupMethod: response,
          loadingYearTopupMethod: false,
          errorYearTopupMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTopupMethod: false }),
        (message: any) => set({ errorYearTopupMethod: message }),
        toast,
      );
    }
  },

  findMonthTopupAmountCard: async (
    toast,
    year: number,
    card_number: string,
  ) => {
    set({ loadingMonthTopupAmount: true, errorMonthTopupAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findMonthTopupAmountByCard(
          token,
          year,
          card_number,
        );

        set({
          monthTopupAmount: response.data,
          loadingMonthTopupAmount: false,
          errorMonthTopupAmount: null,
        });
      } else {
        const response = await TopupService.findMonthTopupAmountCard(
          token,
          year,
          card_number,
        );

        set({
          monthTopupAmount: response,
          loadingMonthTopupAmount: false,
          errorMonthTopupAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTopupAmount: false }),
        (message: any) => set({ errorMonthTopupAmount: message }),
        toast,
      );
    }
  },

  findYearTopupAmountCard: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({ loadingYearTopupAmount: true, errorYearTopupAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findYearTopupAmountByCard(
          token,
          year,
          card_number,
        );

        set({
          yearTopupAmount: response.data,
          loadingYearTopupAmount: false,
          errorYearTopupAmount: null,
        });
      } else {
        const response = await TopupService.findYearTopupAmountCard(
          token,
          year,
          card_number,
        );

        set({
          yearTopupAmount: response,
          loadingYearTopupAmount: false,
          errorYearTopupAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTopupAmount: false }),
        (message: any) => set({ errorYearTopupAmount: message }),
        toast,
      );
    }
  },

  findAllTopups: async (req: FindAllTopup) => {
    set({ loadingGetTopups: true, errorGetTopups: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findAllTopups(token, req);

        set({
          topups: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTopups: false,
          errorGetTopups: null,
        });
      } else {
        const response = await TopupService.findAllTopups(token, req);

        set({
          topups: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTopups: false,
          errorGetTopups: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetTopups: false }),
        (message: any) => set({ errorGetTopups: message }),
        req.toast,
      );
    }
  },

  findByIdTopup: async (req: FindByIdTopup) => {
    set({ loadingGetTopup: true, errorGetTopup: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findByIdTopup(token, req);

        set({
          topup: response.data,
          loadingGetTopup: false,
          errorGetTopup: null,
        });
      } else {
        const response = await TopupService.findByIdTopup(token, req);

        set({
          topup: response,
          loadingGetTopup: false,
          errorGetTopup: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetTopup: false }),
        (message: any) => set({ errorGetTopup: message }),
        req.toast,
      );
    }
  },

  findByActiveTopup: async (req: FindAllTopup) => {
    set({ loadingGetActiveTopup: true, errorGetActiveTopup: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findByActiveTopup(token, req);

        set({
          topups: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetActiveTopup: false,
          errorGetActiveTopup: null,
        });
      } else {
        const response = await TopupService.findByActiveTopup(token, req);

        set({
          topups: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetActiveTopup: false,
          errorGetActiveTopup: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetActiveTopup: false }),
        (message: any) => set({ errorGetActiveTopup: message }),
        null,
      );
    }
  },

  findByCardNumberTopup: async (req: FindByCardNumberTopup) => {
    set({ loadingGetCardNumberTopup: true, errorGetCardNumberTopup: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TopupCommand.findByCardNumberTopup(token, req);

        set({
          topups: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetCardNumberTopup: false,
          errorGetCardNumberTopup: null,
        });
      } else {
        const response = await TopupService.findByCardNumberTopup(token, req);

        set({
          topups: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetCardNumberTopup: false,
          errorGetCardNumberTopup: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetCardNumberTopup: false }),
        (message: any) => set({ errorGetCardNumberTopup: message }),
        req.toast,
      );
    }
  },

  createTopup: async (req: CreateTopup) => {
    set({ loadingCreateTopup: true, errorCreateTopup: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TopupCommand.createTopup(token, req);

        handleMessageAction("topup", "create");

        set({ loadingCreateTopup: false, errorCreateTopup: null });
      } else {
        await TopupService.createTopup(token, req);

        handleMessageAction("topup", "create");

        set({ loadingCreateTopup: false, errorCreateTopup: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingCreateTopup: false }),
        (message: any) => set({ errorCreateTopup: message }),
        req.toast,
      );

      return false;
    }
  },

  updateTopup: async (req: UpdateTopup) => {
    set({ loadingUpdateTopup: true, errorUpdateTopup: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TopupCommand.updateTopup(token, req);

        handleMessageAction("topup", "update");

        set({ loadingUpdateTopup: false, errorUpdateTopup: null });
      } else {
        await TopupService.updateTopup(token, req);

        handleMessageAction("topup", "update");

        set({ loadingUpdateTopup: false, errorUpdateTopup: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingUpdateTopup: false }),
        (message: any) => set({ errorUpdateTopup: message }),
        req.toast,
      );

      return false;
    }
  },
  trashedTopup: async (req: TrashedTopup) => {
    set({ loadingTrashedTopup: true, errorTrashedTopup: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TopupCommand.trashedTopup(token, req);

        handleMessageAction("topup", "trashed");

        set({ loadingTrashedTopup: false, errorTrashedTopup: null });
      } else {
        await TopupService.trashedTopup(token, req);

        handleMessageAction("topup", "trashed");

        set({ loadingTrashedTopup: false, errorTrashedTopup: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingTrashedTopup: false }),
        (message: any) => set({ errorTrashedTopup: message }),
        req.toast,
      );

      return false;
    }
  },
}));

export default useTopupStore;
