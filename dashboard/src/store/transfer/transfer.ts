import { TransferStore } from "@/types/state/transfer/transfer";
import { create } from "zustand";
import { getAccessToken } from "../auth";
import { handleApiError } from "@/helpers/handleApi";
import {
  CreateTransfer,
  FindAllTransfer,
  FindByIdTransfer,
  TransferFrom,
  TransferTo,
  TrashedTransfer,
  UpdateTransfer,
} from "@/types/domain/request";
import TransferCommand from "@/services/ipc/transfer/transfer";
import TransferService from "@/services/api/transfer/transfer";
import { handleMessageAction } from "@/helpers/message";
import { isTauri } from "@tauri-apps/api/core";

const useTransferStore = create<TransferStore>((set, _get) => ({
  transfers: null,
  transfer: null,

  monthStatusSuccess: null,
  yearStatusSuccess: null,

  monthStatusFailed: null,
  yearStatusFailed: null,

  monthTransferAmount: null,
  yearTransferAmount: null,

  monthTransferAmountSender: null,
  yearTransferAmountSender: null,

  monthTransferAmountReceiver: null,
  yearTransferAmountReceiver: null,

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
  loadingMonthTransferAmount: false,
  loadingYearTransferAmount: false,

  loadingMonthTransferAmountSender: false,
  loadingYearTransferAmountSender: false,

  loadingMonthTransferAmountReceiver: false,
  loadingYearTransferAmountReceiver: false,

  loadingGetTransfers: false,
  loadingGetTransfer: false,
  loadingGetTransferFrom: false,
  loadingGetTransferTo: false,
  loadingGetActiveTransfer: false,
  loadingGetTrashedTransfer: false,

  loadingCreateTransfer: false,
  loadingUpdateTransfer: false,
  loadingTrashedTransfer: false,
  loadingRestoreTransfer: false,
  loadingPermanentTransfer: false,

  errorMonthStatusSuccess: null,
  errorYearStatusSuccess: null,
  errorMonthStatusFailed: null,
  errorYearStatusFailed: null,
  errorMonthTransferAmount: null,
  errorYearTransferAmount: null,

  errorMonthTransferAmountSender: null,
  errorYearTransferAmountSender: null,

  errorMonthTransferAmountReceiver: null,
  errorYearTransferAmountReceiver: null,

  errorGetTransfers: null,
  errorGetTransfer: null,
  errorGetTransferFrom: null,
  errorGetTransferTo: null,
  errorGetActiveTransfer: null,
  errorGetTrashedTransfer: null,

  errorCreateTransfer: null,
  errorUpdateTransfer: null,
  errorTrashedTransfer: null,
  errorRestoreTransfer: null,
  errorPermanentTransfer: null,

  setLoadingMonthStatusSuccess: (value) =>
    set({ loadingMonthStatusSuccess: value }),
  setLoadingYearStatusSuccess: (value) =>
    set({ loadingYearStatusSuccess: value }),
  setLoadingMonthStatusFailed: (value) =>
    set({ loadingMonthStatusFailed: value }),
  setLoadingYearStatusFailed: (value) =>
    set({ loadingYearStatusFailed: value }),
  setLoadingMonthTransferAmount: (value) =>
    set({ loadingMonthTransferAmount: value }),
  setLoadingYearTransferAmount: (value) =>
    set({ loadingYearTransferAmount: value }),

  setLoadingMonthTransferAmountSender: (value) =>
    set({ loadingMonthTransferAmountSender: value }),
  setLoadingYearTransferAmountSender: (value) =>
    set({ loadingYearTransferAmountSender: value }),

  setLoadingMonthTransferAmountReceiver: (value) =>
    set({ loadingMonthTransferAmountReceiver: value }),
  setLoadingYearTransferAmountReceiver: (value) =>
    set({ loadingYearTransferAmountReceiver: value }),

  setLoadingGetTransfers: (value) => set({ loadingGetTransfers: value }),
  setLoadingGetTransfer: (value) => set({ loadingGetTransfer: value }),
  setLoadingGetTransferFrom: (value) => set({ loadingGetTransferFrom: value }),
  setLoadingGetTransferTo: (value) => set({ loadingGetTransferTo: value }),
  setLoadingGetActiveTransfer: (value) =>
    set({ loadingGetActiveTransfer: value }),
  setLoadingGetTrashedTransfer: (value) =>
    set({ loadingGetTrashedTransfer: value }),

  setLoadingCreateTransfer: (value) => set({ loadingCreateTransfer: value }),
  setLoadingUpdateTransfer: (value) => set({ loadingUpdateTransfer: value }),
  setLoadingTrashedTransfer: (value) => set({ loadingTrashedTransfer: value }),

  setErrorMonthStatusSuccess: (value) =>
    set({ errorMonthStatusSuccess: value }),
  setErrorYearStatusSuccess: (value) => set({ errorYearStatusSuccess: value }),
  setErrorMonthStatusFailed: (value) => set({ errorMonthStatusFailed: value }),
  setErrorYearStatusFailed: (value) => set({ errorYearStatusFailed: value }),
  setErrorMonthTransferAmount: (value) =>
    set({ errorMonthTransferAmount: value }),
  setErrorYearTransferAmount: (value) =>
    set({ errorYearTransferAmount: value }),

  setErrorMonthTransferAmountSender: (value) =>
    set({ errorMonthTransferAmountSender: value }),
  setErrorYearTransferAmountSender: (value) =>
    set({ errorYearTransferAmountSender: value }),

  setErrorMonthTransferAmountReceiver: (value) =>
    set({ errorMonthTransferAmountReceiver: value }),
  setErrorYearTransferAmountReceiver: (value) =>
    set({ errorYearTransferAmountReceiver: value }),

  setErrorGetTransfers: (value) => set({ errorGetTransfers: value }),
  setErrorGetTransfer: (value) => set({ errorGetTransfer: value }),
  setErrorGetTransferFrom: (value) => set({ errorGetTransferFrom: value }),
  setErrorGetTransferTo: (value) => set({ errorGetTransferTo: value }),
  setErrorGetActiveTransfer: (value) => set({ errorGetActiveTransfer: value }),
  setErrorGetTrashedTransfer: (value) =>
    set({ errorGetTrashedTransfer: value }),

  setErrorCreateTransfer: (value) => set({ errorCreateTransfer: value }),
  setErrorUpdateTransfer: (value) => set({ errorUpdateTransfer: value }),
  setErrorTrashedTransfer: (value) => set({ errorTrashedTransfer: value }),

  findMonthStatusSuccess: async (toast: any, year: number, month: number) => {
    set({ loadingMonthStatusSuccess: true, errorMonthStatusSuccess: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findMonthStatusSuccessTransfer(
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
        const response = await TransferService.findMonthStatusSuccess(
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
        const response = await TransferCommand.findYearStatusSuccessTransfer(
          token,
          year,
        );

        set({
          yearStatusSuccess: response.data,
          loadingYearStatusSuccess: false,
          errorYearStatusSuccess: null,
        });
      } else {
        const response = await TransferService.findYearStatusSuccess(
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
        const response = await TransferCommand.findMonthStatusFailedTransfer(
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
        const response = await TransferService.findMonthStatusFailed(
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
        const response = await TransferCommand.findYearStatusFailedTransfer(
          token,
          year,
        );

        set({
          yearStatusFailed: response.data,
          loadingYearStatusFailed: false,
          errorYearStatusFailed: null,
        });
      } else {
        const response = await TransferService.findYearStatusFailed(
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
          await TransferCommand.findMonthStatusSuccessTransferByCardNumber(
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
          await TransferService.findMonthStatusSuccessByCardNumber(
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
          await TransferCommand.findYearStatusSuccessTransferByCardNumber(
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
          await TransferService.findYearStatusSuccessByCardNumber(
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
          await TransferCommand.findMonthStatusFailedTransferByCardNumber(
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
          await TransferService.findMonthStatusFailedByCardNumber(
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
          await TransferCommand.findYearStatusFailedTransferByCardNumber(
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
        const response = await TransferService.findYearStatusFailedByCardNumber(
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

  findMonthTransferAmount: async (toast: any, year: number) => {
    set({ loadingMonthTransferAmount: true, errorMonthTransferAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findMonthTransferAmountTransfer(
          token,
          year,
        );
        set({
          monthTransferAmount: response.data,
          loadingMonthTransferAmount: false,
          errorMonthTransferAmount: null,
        });
      } else {
        const response = await TransferService.findMonthTransferAmount(
          token,
          year,
        );
        set({
          monthTransferAmount: response,
          loadingMonthTransferAmount: false,
          errorMonthTransferAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTransferAmount: false }),
        (message: any) => set({ errorMonthTransferAmount: message }),
        toast,
      );
    }
  },

  findYearTransferAmount: async (toast: any, year: number) => {
    set({ loadingYearTransferAmount: true, errorYearTransferAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findYearTransferAmountTransfer(
          token,
          year,
        );

        set({
          yearTransferAmount: response.data,
          loadingYearTransferAmount: false,
          errorYearTransferAmount: null,
        });
      } else {
        const response = await TransferService.findYearTransferAmount(
          token,
          year,
        );

        set({
          yearTransferAmount: response,
          loadingYearTransferAmount: false,
          errorYearTransferAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTransferAmount: false }),
        (message: any) => set({ errorYearTransferAmount: message }),
        toast,
      );
    }
  },

  findMonthTransferAmountBySender: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({
      loadingMonthTransferAmountSender: true,
      errorMonthTransferAmountSender: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findMonthTransferAmountBySender(
          token,
          year,
          card_number,
        );

        set({
          monthTransferAmountSender: response.data,
          loadingMonthTransferAmountSender: false,
          errorMonthTransferAmountSender: null,
        });
      } else {
        const response = await TransferService.findMonthTransferAmountBySender(
          token,
          year,
          card_number,
        );

        set({
          monthTransferAmountSender: response,
          loadingMonthTransferAmountSender: false,
          errorMonthTransferAmountSender: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTransferAmountSender: false }),
        (message: any) => set({ errorMonthTransferAmountSender: message }),
        toast,
      );
    }
  },

  findYearTransferAmountBySender: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({
      loadingYearTransferAmountSender: true,
      errorYearTransferAmountSender: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findYearTransferAmountBySender(
          token,
          year,
          card_number,
        );

        set({
          yearTransferAmountSender: response.data,
          loadingYearTransferAmountSender: false,
          errorYearTransferAmountSender: null,
        });
      } else {
        const response = await TransferService.findYearTransferAmountBySender(
          token,
          year,
          card_number,
        );

        set({
          yearTransferAmountSender: response,
          loadingYearTransferAmountSender: false,
          errorYearTransferAmountSender: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTransferAmountSender: false }),
        (message: any) => set({ errorYearTransferAmountSender: message }),
        toast,
      );
    }
  },

  findMonthTransferAmountByReceiver: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({
      loadingMonthTransferAmountReceiver: true,
      errorMonthTransferAmountReceiver: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await TransferCommand.findMonthTransferAmountByReceiver(
            token,
            year,
            card_number,
          );

        set({
          monthTransferAmountReceiver: response.data,
          loadingMonthTransferAmountReceiver: false,
          errorMonthTransferAmountReceiver: null,
        });
      } else {
        const response =
          await TransferService.findMonthTransferAmountByReceiver(
            token,
            year,
            card_number,
          );

        set({
          monthTransferAmountReceiver: response,
          loadingMonthTransferAmountReceiver: false,
          errorMonthTransferAmountReceiver: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTransferAmountReceiver: false }),
        (message: any) => set({ errorMonthTransferAmountReceiver: message }),
        toast,
      );
    }
  },

  findYearTransferAmountByReceiver: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({
      loadingYearTransferAmountReceiver: true,
      errorYearTransferAmountReceiver: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findYearTransferAmountByReceiver(
          token,
          year,
          card_number,
        );

        set({
          yearTransferAmountReceiver: response.data,
          loadingYearTransferAmountReceiver: false,
          errorYearTransferAmountReceiver: null,
        });
      } else {
        const response = await TransferService.findYearTransferAmountByReceiver(
          token,
          year,
          card_number,
        );

        set({
          yearTransferAmountReceiver: response,
          loadingYearTransferAmountReceiver: false,
          errorYearTransferAmountReceiver: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTransferAmountReceiver: false }),
        (message: any) => set({ errorYearTransferAmountReceiver: message }),
        toast,
      );
    }
  },

  findAllTransfers: async (req: FindAllTransfer) => {
    set({ loadingGetTransfers: true, errorGetTransfers: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findAllTransfers(token, req);
        set({
          transfers: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTransfers: false,
          errorGetTransfers: null,
        });
      } else {
        const response = await TransferService.findAllTransfers(token, req);
        set({
          transfers: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTransfers: false,
          errorGetTransfers: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetTransfers: false }),
        (message: any) => set({ errorGetTransfers: message }),
        req.toast,
      );
    }
  },

  findByIdTransfer: async (req: FindByIdTransfer) => {
    set({ loadingGetTransfer: true, errorGetTransfer: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findByIdTransfer(token, req);

        set({
          transfer: response.data,
          loadingGetTransfer: false,
          errorGetTransfer: null,
        });
      } else {
        const response = await TransferService.findByIdTransfer(token, req);

        set({
          transfer: response.data,
          loadingGetTransfer: false,
          errorGetTransfer: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetTransfer: false }),
        (message: any) => set({ errorGetTransfer: message }),
        req.toast,
      );
    }
  },

  findByTransferFrom: async (req: TransferFrom) => {
    set({ loadingGetTransferFrom: true, errorGetTransferFrom: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findByTransferFrom(token, req);

        set({
          transfers: response.data,
          loadingGetTransferFrom: false,
          errorGetTransferFrom: null,
        });
      } else {
        const response = await TransferService.findByTransferFrom(token, req);

        set({
          transfers: response,
          loadingGetTransferFrom: false,
          errorGetTransferFrom: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetTransferFrom: false }),
        (message: any) => set({ errorGetTransferFrom: message }),
        req.toast,
      );
    }
  },

  findByTransferTo: async (req: TransferTo) => {
    set({ loadingGetTransferTo: true, errorGetTransferTo: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findByTransferTo(token, req);

        set({
          transfers: response.data,
          loadingGetTransferTo: false,
          errorGetTransferTo: null,
        });
      } else {
        const response = await TransferService.findByTransferTo(token, req);

        set({
          transfers: response,
          loadingGetTransferTo: false,
          errorGetTransferTo: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetTransferTo: false }),
        (message: any) => set({ errorGetTransferTo: message }),
        req.toast,
      );
    }
  },

  findByActiveTransfer: async (req: FindAllTransfer) => {
    set({ loadingGetActiveTransfer: true, errorGetActiveTransfer: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransferCommand.findByActiveTransfer(token, req);

        set({
          transfers: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetActiveTransfer: false,
          errorGetActiveTransfer: null,
        });
      } else {
        const response = await TransferService.findByActiveTransfer(token, req);

        set({
          transfers: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetActiveTransfer: false,
          errorGetActiveTransfer: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetActiveTransfer: false }),
        (message: any) => set({ errorGetActiveTransfer: message }),
        null,
      );
    }
  },

  createTransfer: async (req: CreateTransfer) => {
    set({ loadingCreateTransfer: true, errorCreateTransfer: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TransferCommand.createTransfer(token, req);

        handleMessageAction("transfer", "create");

        set({ loadingCreateTransfer: false, errorCreateTransfer: null });
      } else {
        await TransferService.createTransfer(token, req);

        handleMessageAction("transfer", "create");

        set({ loadingCreateTransfer: false, errorCreateTransfer: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingCreateTransfer: false }),
        (message: any) => set({ errorCreateTransfer: message }),
        req.toast,
      );
      return false;
    }
  },

  updateTransfer: async (req: UpdateTransfer) => {
    set({ loadingUpdateTransfer: true, errorUpdateTransfer: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TransferCommand.updateTransfer(token, req);

        handleMessageAction("transfer", "update");

        set({ loadingUpdateTransfer: false, errorUpdateTransfer: null });
      } else {
        await TransferService.updateTransfer(token, req);

        handleMessageAction("transfer", "update");

        set({ loadingUpdateTransfer: false, errorUpdateTransfer: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingUpdateTransfer: false }),
        (message: any) => set({ errorUpdateTransfer: message }),
        req.toast,
      );
      return false;
    }
  },

  trashedTransfer: async (req: TrashedTransfer) => {
    set({ loadingTrashedTransfer: true, errorTrashedTransfer: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TransferCommand.trashedTransfer(token, req);

        handleMessageAction("transfer", "trashed");

        set({ loadingTrashedTransfer: false, errorTrashedTransfer: null });
      } else {
        await TransferService.trashedTransfer(token, req);

        handleMessageAction("transfer", "trashed");

        set({ loadingTrashedTransfer: false, errorTrashedTransfer: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingTrashedTransfer: false }),
        (message: any) => set({ errorTrashedTransfer: message }),
        req.toast,
      );

      return false;
    }
  },
}));

export default useTransferStore;
