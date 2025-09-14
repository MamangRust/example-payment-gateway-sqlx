import { TransactionStore } from "@/types/state/transaction/transaction";
import { create } from "zustand";
import { getAccessToken } from "../auth";
import { handleApiError } from "@/helpers/handleApi";
import {
  CreateTransaction,
  FindAllTransaction,
  FindyByCardNumberTransaction,
  FindyByIdTransaction,
  FindyByMerchantTransaction,
  TrashedTransaction,
  UpdateTransaction,
} from "@/types/domain/request";
import TransactionCommand from "@/services/ipc/transaction/transaction";
import TransactionService from "@/services/api/transaction/transaction";
import { handleMessageAction } from "@/helpers/message";
import { isTauri } from "@tauri-apps/api/core";

const useTransactionStore = create<TransactionStore>((set, _get) => ({
  transactions: null,
  transaction: null,

  monthStatusSuccess: null,
  yearStatusSuccess: null,

  monthStatusFailed: null,
  yearStatusFailed: null,

  monthTransactionMethod: null,
  yearTransactionMethod: null,

  monthTransactionAmount: null,
  yearTransactionAmount: null,

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
  loadingMonthTransactionMethod: false,
  loadingYearTransactionMethod: false,
  loadingMonthTransactionAmount: false,
  loadingYearTransactionAmount: false,

  loadingGetTransactions: false,
  loadingGetTransaction: false,
  loadingGetCardNumberTransaction: false,
  loadingGetMerchantTransaction: false,
  loadingGetActiveTransaction: false,
  loadingGetTrashedTransaction: false,

  loadingCreateTransaction: false,
  loadingUpdateTransaction: false,
  loadingRestoreTransaction: false,
  loadingTrashedTransaction: false,
  loadingDeletePermanentTransaction: false,

  errorMonthStatusSuccess: null,
  errorYearStatusSuccess: null,
  errorMonthStatusFailed: null,
  errorYearStatusFailed: null,
  errorMonthTransactionMethod: null,
  errorYearTransactionMethod: null,
  errorMonthTransactionAmount: null,
  errorYearTransactionAmount: null,

  errorGetTransactions: null,
  errorGetTransaction: null,
  errorGetCardNumberTransaction: null,
  errorGetMerchantTransaction: null,
  errorGetActiveTransaction: null,
  errorGetTrashedTransaction: null,

  errorCreateTransaction: null,
  errorUpdateTransaction: null,
  errorRestoreTransaction: null,
  errorTrashedTransaction: null,
  errorDeletePermanentTransaction: null,

  setLoadingMonthStatusSuccess: (value) =>
    set({ loadingMonthStatusSuccess: value }),
  setLoadingYearStatusSuccess: (value) =>
    set({ loadingYearStatusSuccess: value }),
  setLoadingMonthStatusFailed: (value) =>
    set({ loadingMonthStatusFailed: value }),
  setLoadingYearStatusFailed: (value) =>
    set({ loadingYearStatusFailed: value }),

  setLoadingMonthTransactionMethod: (value) =>
    set({ loadingMonthTransactionMethod: value }),
  setLoadingYearTransactionMethod: (value) =>
    set({ loadingYearTransactionMethod: value }),
  setLoadingMonthTransactionAmount: (value) =>
    set({ loadingMonthTransactionAmount: value }),
  setLoadingYearTransactionAmount: (value) =>
    set({ loadingYearTransactionAmount: value }),

  setLoadingGetTransactions: (value) => set({ loadingGetTransactions: value }),
  setLoadingGetTransaction: (value) => set({ loadingGetTransaction: value }),
  setLoadingGetCardNumberTransaction: (value) =>
    set({ loadingGetCardNumberTransaction: value }),
  setLoadingGetMerchantTransaction: (value) =>
    set({ loadingGetMerchantTransaction: value }),
  setLoadingGetActiveTransaction: (value) =>
    set({ loadingGetActiveTransaction: value }),
  setLoadingGetTrashedTransaction: (value) =>
    set({ loadingGetTrashedTransaction: value }),

  setLoadingCreateTransaction: (value) =>
    set({ loadingCreateTransaction: value }),
  setLoadingUpdateTransaction: (value) =>
    set({ loadingUpdateTransaction: value }),
  setLoadingTrashedTransaction: (value) =>
    set({ loadingTrashedTransaction: value }),

  setErrorMonthStatusSuccess: (value) =>
    set({ errorMonthStatusSuccess: value }),
  setErrorYearStatusSuccess: (value) => set({ errorYearStatusSuccess: value }),
  setErrorMonthStatusFailed: (value) => set({ errorMonthStatusFailed: value }),
  setErrorYearStatusFailed: (value) => set({ errorYearStatusFailed: value }),

  setErrorMonthTransactionMethod: (value) =>
    set({ errorMonthTransactionMethod: value }),
  setErrorYearTransactionMethod: (value) =>
    set({ errorYearTransactionMethod: value }),
  setErrorMonthTransactionAmount: (value) =>
    set({ errorMonthTransactionAmount: value }),
  setErrorYearTransactionAmount: (value) =>
    set({ errorYearTransactionAmount: value }),

  setErrorGetTransactions: (value) => set({ errorGetTransactions: value }),
  setErrorGetTransaction: (value) => set({ errorGetTransaction: value }),
  setErrorGetCardNumberTransaction: (value) =>
    set({ errorGetCardNumberTransaction: value }),
  setErrorGetMerchantTransaction: (value) =>
    set({ errorGetMerchantTransaction: value }),
  setErrorGetActiveTransaction: (value) =>
    set({ errorGetActiveTransaction: value }),
  setErrorGetTrashedTransaction: (value) =>
    set({ errorGetTrashedTransaction: value }),

  setErrorCreateTransaction: (value) => set({ errorCreateTransaction: value }),
  setErrorUpdateTransaction: (value) => set({ errorUpdateTransaction: value }),
  setErrorTrashedTransaction: (value) =>
    set({ errorTrashedTransaction: value }),

  findMonthStatusSuccess: async (toast: any, year: number, month: number) => {
    set({ loadingMonthStatusSuccess: true, errorMonthStatusSuccess: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await TransactionCommand.findMonthStatusSuccessTransaction(
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
        const response = await TransactionService.findMonthStatusSuccess(
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
        const response =
          await TransactionCommand.findYearStatusSuccessTransaction(
            token,
            year,
          );

        set({
          yearStatusSuccess: response.data,
          loadingYearStatusSuccess: false,
          errorYearStatusSuccess: null,
        });
      } else {
        const response = await TransactionService.findYearStatusSuccess(
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
        const response =
          await TransactionCommand.findMonthStatusFailedTransaction(
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
        const response = await TransactionService.findMonthStatusFailed(
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
        const response =
          await TransactionCommand.findYearStatusFailedTransaction(token, year);

        set({
          yearStatusFailed: response.data,
          loadingYearStatusFailed: false,
          errorYearStatusFailed: null,
        });
      } else {
        const response = await TransactionService.findYearStatusFailed(
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
    card_number: string,
  ) => {
    set({ loadingMonthStatusSuccess: true, errorMonthStatusSuccess: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await TransactionCommand.findMonthStatusSuccessTransactionByCardNumber(
            token,
            year,
            month,
            card_number,
          );

        set({
          monthStatusSuccess: response.data,
          loadingMonthStatusSuccess: false,
          errorMonthStatusSuccess: null,
        });
      } else {
        const response =
          await TransactionService.findMonthStatusSuccessByCardNumber(
            token,
            year,
            month,
            card_number,
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
    card_number: string,
  ) => {
    set({ loadingYearStatusSuccess: true, errorYearStatusSuccess: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await TransactionCommand.findYearStatusSuccessTransactionByCardNumber(
            token,
            year,
            card_number,
          );

        set({
          yearStatusSuccess: response.data,
          loadingYearStatusSuccess: false,
          errorYearStatusSuccess: null,
        });
      } else {
        const response =
          await TransactionService.findYearStatusSuccessByCardNumber(
            token,
            year,
            card_number,
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
    card_number: string,
  ) => {
    set({ loadingMonthStatusFailed: true, errorMonthStatusFailed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await TransactionCommand.findMonthStatusFailedTransactionByCardNumber(
            token,
            year,
            month,
            card_number,
          );

        set({
          monthStatusFailed: response.data,
          loadingMonthStatusFailed: false,
          errorMonthStatusFailed: null,
        });
      } else {
        const response =
          await TransactionService.findMonthStatusFailedByCardNumber(
            token,
            year,
            month,
            card_number,
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
          await TransactionCommand.findYearStatusFailedTransactionByCardNumber(
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
        const response =
          await TransactionService.findYearStatusFailedByCardNumber(
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

  findMonthTransactionMethod: async (toast: any, year: number) => {
    set({
      loadingMonthTransactionMethod: true,
      errorMonthTransactionMethod: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransactionCommand.findMonthTransactionMethod(
          token,
          year,
        );

        set({
          monthTransactionMethod: response.data,
          loadingMonthTransactionMethod: false,
          errorMonthTransactionMethod: null,
        });
      } else {
        const response = await TransactionService.findMonthTransactionMethod(
          token,
          year,
        );

        set({
          monthTransactionMethod: response,
          loadingMonthTransactionMethod: false,
          errorMonthTransactionMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTransactionMethod: false }),
        (message: any) => set({ errorMonthTransactionMethod: message }),
        toast,
      );
    }
  },

  findYearTransactionMethod: async (toast: any, year: number) => {
    set({
      loadingYearTransactionMethod: true,
      errorYearTransactionMethod: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransactionCommand.findYearTransactionMethod(
          token,
          year,
        );

        set({
          yearTransactionMethod: response.data,
          loadingYearTransactionMethod: false,
          errorYearTransactionMethod: null,
        });
      } else {
        const response = await TransactionService.findYearTransactionMethod(
          token,
          year,
        );

        set({
          yearTransactionMethod: response,
          loadingYearTransactionMethod: false,
          errorYearTransactionMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTransactionMethod: false }),
        (message: any) => set({ errorYearTransactionMethod: message }),
        toast,
      );
    }
  },
  findMonthTransactionAmount: async (toast: any, year: number) => {
    set({
      loadingMonthTransactionAmount: true,
      errorMonthTransactionAmount: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransactionCommand.findMonthTransactionMethod(
          token,
          year,
        );

        set({
          monthTransactionAmount: response.data,
          loadingMonthTransactionAmount: false,
          errorMonthTransactionAmount: null,
        });
      } else {
        const response = await TransactionService.findMonthTransactionMethod(
          token,
          year,
        );

        set({
          monthTransactionAmount: response,
          loadingMonthTransactionAmount: false,
          errorMonthTransactionAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTransactionAmount: false }),
        (message: any) => set({ errorMonthTransactionAmount: message }),
        toast,
      );
    }
  },

  findYearTransactionAmount: async (toast: any, year: number) => {
    set({
      loadingYearTransactionAmount: true,
      errorYearTransactionAmount: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransactionCommand.findYearTransactionMethod(
          token,
          year,
        );

        set({
          yearTransactionAmount: response.data,
          loadingYearTransactionAmount: false,
          errorYearTransactionAmount: null,
        });
      } else {
        const response = await TransactionService.findYearTransactionMethod(
          token,
          year,
        );

        set({
          yearTransactionAmount: response,
          loadingYearTransactionAmount: false,
          errorYearTransactionAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTransactionAmount: false }),
        (message: any) => set({ errorYearTransactionAmount: message }),
        toast,
      );
    }
  },

  findMonthTransactionMethodCard: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({
      loadingMonthTransactionMethod: true,
      errorMonthTransactionMethod: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await TransactionCommand.findMonthTransactionMethodByCard(
            token,
            year,
            card_number,
          );

        set({
          monthTransactionMethod: response.data,
          loadingMonthTransactionMethod: false,
          errorMonthTransactionMethod: null,
        });
      } else {
        const response =
          await TransactionService.findMonthTransactionMethodCard(
            token,
            year,
            card_number,
          );

        set({
          monthTransactionMethod: response,
          loadingMonthTransactionMethod: false,
          errorMonthTransactionMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTransactionMethod: false }),
        (message: any) => set({ errorMonthTransactionMethod: message }),
        toast,
      );
    }
  },

  findYearTransactionMethodCard: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({
      loadingYearTransactionMethod: true,
      errorYearTransactionMethod: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await TransactionCommand.findYearTransactionMethodByCard(
            token,
            year,
            card_number,
          );

        set({
          yearTransactionMethod: response.data,
          loadingYearTransactionMethod: false,
          errorYearTransactionMethod: null,
        });
      } else {
        const response = await TransactionService.findYearTransactionMethodCard(
          token,
          year,
          card_number,
        );

        set({
          yearTransactionMethod: response,
          loadingYearTransactionMethod: false,
          errorYearTransactionMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTransactionMethod: false }),
        (message: any) => set({ errorYearTransactionMethod: message }),
        toast,
      );
    }
  },
  findMonthTransactionAmountCard: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({
      loadingMonthTransactionAmount: true,
      errorMonthTransactionAmount: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await TransactionCommand.findMonthTransactionAmountByCard(
            token,
            year,
            card_number,
          );

        set({
          monthTransactionAmount: response.data,
          loadingMonthTransactionAmount: false,
          errorMonthTransactionAmount: null,
        });
      } else {
        const response =
          await TransactionService.findMonthTransactionAmountCard(
            token,
            year,
            card_number,
          );

        set({
          monthTransactionAmount: response,
          loadingMonthTransactionAmount: false,
          errorMonthTransactionAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTransactionAmount: false }),
        (message: any) => set({ errorMonthTransactionAmount: message }),
        toast,
      );
    }
  },

  findYearTransactionAmountCard: async (
    toast: any,
    year: number,
    card_number: string,
  ) => {
    set({
      loadingYearTransactionAmount: true,
      errorYearTransactionAmount: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await TransactionCommand.findYearTransactionAmountByCard(
            token,
            year,
            card_number,
          );

        set({
          yearTransactionAmount: response.data,
          loadingYearTransactionAmount: false,
          errorYearTransactionAmount: null,
        });
      } else {
        const response = await TransactionService.findYearTransactionAmountCard(
          token,
          year,
          card_number,
        );

        set({
          yearTransactionAmount: response,
          loadingYearTransactionAmount: false,
          errorYearTransactionAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTransactionAmount: false }),
        (message: any) => set({ errorYearTransactionAmount: message }),
        toast,
      );
    }
  },

  findAllTransactions: async (req: FindAllTransaction) => {
    set({ loadingGetTransactions: true, errorGetTransactions: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransactionCommand.findAllTransactions(
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
          loadingGetTransactions: false,
          errorGetTransactions: null,
        });
      } else {
        const response = await TransactionService.findAllTransactions(
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
          loadingGetTransactions: false,
          errorGetTransactions: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetTransactions: false }),
        (message: any) => set({ errorGetTransactions: message }),
        req.toast,
      );
    }
  },

  findByIdTransaction: async (req: FindyByIdTransaction) => {
    set({ loadingGetTransaction: true, errorGetTransaction: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransactionCommand.findByIdTransaction(
          token,
          req,
        );

        set({
          transaction: response.data,
          loadingGetTransaction: false,
          errorGetTransaction: null,
        });
      } else {
        const response = await TransactionService.findByIdTransaction(
          token,
          req,
        );

        set({
          transaction: response,
          loadingGetTransaction: false,
          errorGetTransaction: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetTransaction: false }),
        (message: any) => set({ errorGetTransaction: message }),
        req.toast,
      );
    }
  },

  findByCardNumberTransaction: async (req: FindyByCardNumberTransaction) => {
    set({
      loadingGetCardNumberTransaction: true,
      errorGetCardNumberTransaction: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransactionCommand.findByCardNumberTransaction(
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
          loadingGetCardNumberTransaction: false,
          errorGetCardNumberTransaction: null,
        });
      } else {
        const response = await TransactionService.findByCardNumberTransaction(
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
          loadingGetCardNumberTransaction: false,
          errorGetCardNumberTransaction: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetCardNumberTransaction: false }),
        (message: any) => set({ errorGetCardNumberTransaction: message }),
        req.cardNumber,
      );
    }
  },

  findByMerchantTransaction: async (req: FindyByMerchantTransaction) => {
    set({
      loadingGetMerchantTransaction: true,
      errorGetMerchantTransaction: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransactionCommand.findByMerchantTransaction(
          token,
          req,
        );

        set({
          transactions: response.data,
          loadingGetMerchantTransaction: false,
          errorGetMerchantTransaction: null,
        });
      } else {
        const response = await TransactionService.findByMerchantTransaction(
          token,
          req,
        );

        set({
          transactions: response,
          loadingGetMerchantTransaction: false,
          errorGetMerchantTransaction: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetMerchantTransaction: false }),
        (message: any) => set({ errorGetMerchantTransaction: message }),
        req.toast,
      );
    }
  },

  findByActiveTransaction: async (req: FindAllTransaction) => {
    set({ loadingGetActiveTransaction: true, errorGetActiveTransaction: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await TransactionCommand.findByActiveTransaction(
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
          loadingGetActiveTransaction: false,
          errorGetActiveTransaction: null,
        });
      } else {
        const response = await TransactionService.findByActiveTransaction(
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
          loadingGetActiveTransaction: false,
          errorGetActiveTransaction: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetActiveTransaction: false }),
        (message: any) => set({ errorGetActiveTransaction: message }),
        null,
      );
    }
  },

  createTransaction: async (req: CreateTransaction) => {
    set({ loadingCreateTransaction: true, errorCreateTransaction: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TransactionCommand.createTransaction(token, req);

        handleMessageAction("transaction", "create");

        set({ loadingCreateTransaction: false, errorCreateTransaction: null });
      } else {
        await TransactionService.createTransaction(token, req);

        handleMessageAction("transaction", "create");

        set({ loadingCreateTransaction: false, errorCreateTransaction: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingCreateTransaction: false }),
        (message: any) => set({ errorCreateTransaction: message }),
        req.toast,
      );
      return false;
    }
  },

  updateTransaction: async (req: UpdateTransaction) => {
    set({ loadingUpdateTransaction: true, errorUpdateTransaction: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TransactionCommand.updateTransaction(token, req);
        handleMessageAction("transaction", "update");

        set({ loadingUpdateTransaction: false, errorUpdateTransaction: null });
      } else {
        await TransactionService.updateTransaction(token, req);
        handleMessageAction("transaction", "update");

        set({ loadingUpdateTransaction: false, errorUpdateTransaction: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingUpdateTransaction: false }),
        (message: any) => set({ errorUpdateTransaction: message }),
        req.toast,
      );
      return false;
    }
  },

  trashedTransaction: async (req: TrashedTransaction) => {
    set({ loadingTrashedTransaction: true, errorTrashedTransaction: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await TransactionCommand.trashedTransaction(token, req);

        handleMessageAction("transaction", "trashed");

        set({
          loadingTrashedTransaction: false,
          errorTrashedTransaction: null,
        });
      } else {
        await TransactionService.trashedTransaction(token, req);

        handleMessageAction("transaction", "trashed");

        set({
          loadingTrashedTransaction: false,
          errorTrashedTransaction: null,
        });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingTrashedTransaction: false }),
        (message: any) => set({ errorTrashedTransaction: message }),
        req.toast,
      );

      return false;
    }
  },
}));

export default useTransactionStore;
