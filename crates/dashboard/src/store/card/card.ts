import { create } from "zustand";
import { handleApiError } from "@/helpers/handleApi";
import { CardStore } from "@/types/state/card/card";
import {
  CreateCard,
  FindAllCard,
  FindByCardNumber,
  FindByIdCard,
  UpdateCard,
} from "@/types/domain/request";
import { getAccessToken } from "../auth";
import CardService from "@/services/api/card/card";
import CardCommand from "@/services/ipc/card/card";
import { isTauri } from "@tauri-apps/api/core";
import { FindByUser } from "@/types/domain/request/card/user";
import { handleMessageAction } from "@/helpers/message";

const useCardStore = create<CardStore>((set, _get) => ({
  cards: null,
  card: null,

  dashboard: null,
  dashboardCardNumber: null,

  monthBalance: null,
  yearBalance: null,

  monthTopupAmount: null,
  yearTopupAmount: null,

  monthWithdrawAmount: null,
  yearWithdrawAmount: null,

  monthTransactionAmount: null,
  yearTransactionAmount: null,

  monthTransferSender: null,
  yearTransferSender: null,

  monthTransferReceiver: null,
  yearTransferReceiver: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingDashboard: false,
  loadingDashboardCardNumber: false,

  loadingMonthBalance: false,
  loadingYearBalance: false,

  loadingMonthTopupAmount: false,
  loadingYearTopupAmount: false,

  loadingMonthWithdrawAmount: false,
  loadingYearWithdrawAmount: false,

  loadingMonthTransaction: false,
  loadingYearTransaction: false,

  loadingMonthTransferSender: false,
  loadingYearTransferSender: false,

  loadingMonthTransferReceiver: false,
  loadingYearTransferReceiver: false,

  loadingGetCards: false,
  loadingGetCard: false,
  loadingGetCardByUser: false,
  loadingGetActiveCards: false,
  loadingGetTrashedCards: false,
  loadingGetCardByCardNumber: false,
  loadingCreateCard: false,
  loadingUpdateCard: false,
  loadingTrashedCard: false,

  errorDashboard: null,
  errorDashboardCardNumber: null,

  errorMonthBalance: null,
  errorYearBalance: null,

  errorMonthTopupAmount: null,
  errorYearTopupAmount: null,

  errorMonthWithdrawAmount: null,
  errorYearWithdrawAmount: null,

  errorMonthTransaction: null,
  errorYearTransaction: null,

  errorMonthTransferSender: null,
  errorYearTransferSender: null,

  errorMonthTransferReceiver: null,
  errorYearTransferReceiver: null,

  errorGetCards: null,
  errorGetCard: null,
  errorGetCardByUser: null,
  errorGetActiveCards: null,
  errorGetCardByCardNumber: null,
  errorCreateCard: null,
  errorUpdateCard: null,
  errorTrashedCard: null,

  setErrorDashboard: (value: string | null) => set({ errorDashboard: value }),
  setErrorDashboardCardNumber: (value: string | null) =>
    set({ errorDashboardCardNumber: value }),
  setErrorMonthBalance: (value: string | null) =>
    set({ errorMonthBalance: value }),
  setErrorYearBalance: (value: string | null) =>
    set({ errorYearBalance: value }),
  setErrorMonthTopupAmount: (value: string | null) =>
    set({ errorMonthTopupAmount: value }),
  setErrorYearTopupAmount: (value: string | null) =>
    set({ errorYearTopupAmount: value }),
  setErrorMonthWithdrawAmount: (value: string | null) =>
    set({ errorMonthWithdrawAmount: value }),
  setErrorYearWithdrawAmount: (value: string | null) =>
    set({ errorYearWithdrawAmount: value }),
  setErrorMonthTransaction: (value: string | null) =>
    set({ errorMonthTransaction: value }),
  setErrorYearTransaction: (value: string | null) =>
    set({ errorYearTransaction: value }),
  setErrorMonthTransferSender: (value: string | null) =>
    set({ errorMonthTransferSender: value }),
  setErrorYearTransferSender: (value: string | null) =>
    set({ errorYearTransferSender: value }),

  setErrorMonthTransferReceiver: (value: string | null) =>
    set({ errorMonthTransferReceiver: value }),
  setErrorYearTransferReceiver: (value: string | null) =>
    set({ errorYearTransferReceiver: value }),

  setErrorGetCards: (value: string | null) => set({ errorGetCards: value }),
  setErrorGetCard: (value: string | null) => set({ errorGetCard: value }),
  setErrorGetCardByUser: (value: string | null) =>
    set({ errorGetCardByUser: value }),
  setErrorGetActiveCards: (value: string | null) =>
    set({ errorGetActiveCards: value }),

  setErrorGetCardByCardNumber: (value: string | null) =>
    set({ errorGetCardByCardNumber: value }),
  setErrorCreateCard: (value: string | null) => set({ errorCreateCard: value }),
  setErrorUpdateCard: (value: string | null) => set({ errorUpdateCard: value }),
  setErrorTrashedCard: (value: string | null) =>
    set({ errorTrashedCard: value }),

  setLoadingDashboard: (value) => set({ loadingDashboard: value }),
  setLoadingDashboardCardNumber: (value) =>
    set({ loadingDashboardCardNumber: value }),
  setLoadingMonthBalance: (value) => set({ loadingMonthBalance: value }),
  setLoadingYearBalance: (value) => set({ loadingYearBalance: value }),
  setLoadingMonthTopupAmount: (value) =>
    set({ loadingMonthTopupAmount: value }),
  setLoadingYearTopupAmount: (value) => set({ loadingYearTopupAmount: value }),
  setLoadingMonthWithdrawAmount: (value) =>
    set({ loadingMonthWithdrawAmount: value }),
  setLoadingYearWithdrawAmount: (value) =>
    set({ loadingYearWithdrawAmount: value }),
  setLoadingMonthTransaction: (value) =>
    set({ loadingMonthTransaction: value }),
  setLoadingYearTransaction: (value) => set({ loadingYearTransaction: value }),
  setLoadingMonthTransferSender: (value) =>
    set({ loadingMonthTransferSender: value }),
  setLoadingYearTransferSender: (value) =>
    set({ loadingYearTransferSender: value }),
  setLoadingMonthTransferReceiver: (value) =>
    set({ loadingMonthTransferReceiver: value }),
  setLoadingYearTransferReceiver: (value) =>
    set({ loadingYearTransferReceiver: value }),

  setLoadingGetCards: (value: boolean) => set({ loadingGetCards: value }),
  setLoadingGetCard: (value: boolean) => set({ loadingGetCard: value }),
  setLoadingGetCardByUser: (value: boolean) =>
    set({ loadingGetCardByUser: value }),
  setLoadingGetActiveCards: (value: boolean) =>
    set({ loadingGetActiveCards: value }),

  setLoadingGetCardByCardNumber: (value: boolean) =>
    set({ loadingGetCardByCardNumber: value }),
  setLoadingCreateCard: (value: boolean) => set({ loadingCreateCard: value }),
  setLoadingUpdateCard: (value: boolean) => set({ loadingUpdateCard: value }),
  setLoadingTrashedCard: (value: boolean) => set({ loadingTrashedCard: value }),

  dashboardCard: async (toast: any) => {
    set({ loadingDashboard: true, errorDashboard: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findDashboard(token);
        set({
          dashboard: response.data,
          loadingDashboard: false,
          errorDashboard: null,
        });
      } else {
        const response = await CardService.findDashboard(token);

        set({
          dashboard: response,
          loadingDashboard: false,
          errorDashboard: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingDashboard: false }),
        (message: any) => set({ errorDashboard: message }),
        toast,
      );
    }
  },

  dashboardCardCardNumber: async (toast: any, card_number: string) => {
    set({ loadingDashboardCardNumber: true, errorDashboardCardNumber: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findDashboardByCardNumber(
          token,
          card_number,
        );

        set({
          dashboardCardNumber: response.data,
          loadingDashboardCardNumber: false,
          errorDashboardCardNumber: null,
        });
      } else {
        const response = await CardService.findDashboardByCardNumber(
          token,
          card_number,
        );

        set({
          dashboardCardNumber: response,
          loadingDashboardCardNumber: false,
          errorDashboardCardNumber: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingDashboardCardNumber: false }),
        (message: any) => set({ errorDashboardCardNumber: message }),
        toast,
      );
    }
  },

  findMonthBalance: async (toast: any, year: number) => {
    set({ loadingMonthBalance: true, errorMonthBalance: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthBalance(token, year);

        set({
          monthBalance: response.data,
          loadingMonthBalance: false,
          errorMonthBalance: null,
        });
      } else {
        const response = await CardService.findMonthBalance(token, year);

        set({
          monthBalance: response,
          loadingMonthBalance: false,
          errorMonthBalance: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingMonthBalance: false }),
        (message: any) => set({ errorMonthBalance: message }),
        toast,
      );
    }
  },

  findYearBalance: async (toast: any, year: number) => {
    set({ loadingYearBalance: true, errorYearBalance: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findYearBalance(token, year);

        set({
          yearBalance: response.data,
          loadingYearBalance: false,
          errorYearBalance: null,
        });
      } else {
        const response = await CardService.findYearBalance(token, year);

        set({
          yearBalance: response,
          loadingYearBalance: false,
          errorYearBalance: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearBalance: false }),
        (message: any) => set({ errorYearBalance: message }),
        toast,
      );
    }
  },

  findMonthTopupAmount: async (toast: any, year: number) => {
    set({ loadingMonthTopupAmount: true, errorMonthTopupAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthTopupAmountCard(
          token,
          year,
        );

        set({
          monthTopupAmount: response.data,
          loadingMonthTopupAmount: false,
          errorMonthTopupAmount: null,
        });
      } else {
        const response = await CardService.findMonthTopupAmount(token, year);

        set({
          monthTopupAmount: response,
          loadingMonthTopupAmount: false,
          errorMonthTopupAmount: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
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
        const response = await CardCommand.findYearTopupAmountCard(token, year);

        set({
          yearTopupAmount: response.data,
          loadingYearTopupAmount: false,
          errorYearTopupAmount: null,
        });
      } else {
        const response = await CardService.findYearTopupAmount(token, year);

        set({
          yearTopupAmount: response,
          loadingYearTopupAmount: false,
          errorYearTopupAmount: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearTopupAmount: false }),
        (message: any) => set({ errorYearTopupAmount: message }),
        toast,
      );
    }
  },

  findMonthWithdrawAmount: async (toast: any, year: number) => {
    set({ loadingMonthWithdrawAmount: true, errorMonthWithdrawAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthWithdrawAmount(token, year);

        set({
          monthWithdrawAmount: response.data,
          loadingMonthWithdrawAmount: false,
          errorMonthWithdrawAmount: null,
        });
      } else {
        const response = await CardService.findMonthWithdrawAmount(token, year);

        set({
          monthWithdrawAmount: response,
          loadingMonthWithdrawAmount: false,
          errorMonthWithdrawAmount: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
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
        const response = await CardCommand.findYearWithdrawAmount(token, year);

        set({
          yearWithdrawAmount: response.data,
          loadingYearWithdrawAmount: false,
          errorYearWithdrawAmount: null,
        });
      } else {
        const response = await CardService.findYearWithdrawAmount(token, year);

        set({
          yearWithdrawAmount: response,
          loadingYearWithdrawAmount: false,
          errorYearWithdrawAmount: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearWithdrawAmount: false }),
        (message: any) => set({ errorYearWithdrawAmount: message }),
        toast,
      );
    }
  },

  findMonthlyTransferSenderAmount: async (toast: any, year: number) => {
    set({
      loadingMonthTransferSender: true,
      errorMonthTransferSender: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthTransferSenderAmount(
          token,
          year,
        );

        set({
          monthTransferSender: response.data,
          loadingMonthTransferSender: false,
          errorMonthTransferSender: null,
        });
      } else {
        const response = await CardService.findMonthlyTransferSenderAmount(
          token,
          year,
        );

        set({
          monthTransferSender: response,
          loadingMonthTransferSender: false,
          errorMonthTransferSender: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingMonthTransferSender: false }),
        (message: any) => set({ errorMonthTransferSender: message }),
        toast,
      );
    }
  },

  findYearlyTransferSenderAmount: async (toast: any, year: number) => {
    set({
      loadingYearTransferSender: true,
      errorYearTransferSender: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findYearTransferSenderAmount(
          token,
          year,
        );

        set({
          yearTransferSender: response.data,
          loadingYearTransferSender: false,
          errorYearTransferSender: null,
        });
      } else {
        const response = await CardService.findYearlyTransferSenderAmount(
          token,
          year,
        );

        set({
          yearTransferSender: response,
          loadingYearTransferSender: false,
          errorYearTransferSender: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearTransferSender: false }),
        (message: any) => set({ errorYearTransferSender: message }),
        toast,
      );
    }
  },

  findMonthlyTransferReceiverAmount: async (toast: any, year: number) => {
    set({
      loadingMonthTransferReceiver: true,
      errorMonthTransferReceiver: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthTransferReceiverAmount(
          token,
          year,
        );

        set({
          monthTransferReceiver: response.data,
          loadingMonthTransferReceiver: false,
          errorMonthTransferReceiver: null,
        });
      } else {
        const response = await CardService.findMonthlyTransferReceiverAmount(
          token,
          year,
        );

        set({
          monthTransferReceiver: response,
          loadingMonthTransferReceiver: false,
          errorMonthTransferReceiver: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingMonthTransferReceiver: false }),
        (message: any) => set({ errorMonthTransferReceiver: message }),
        toast,
      );
    }
  },

  findYearlyTransferReceiverAmount: async (toast: any, year: number) => {
    set({
      loadingYearTransferReceiver: true,
      errorYearTransferReceiver: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findYearTransferReceiverAmount(
          token,
          year,
        );

        set({
          yearTransferReceiver: response.data,
          loadingYearTransferReceiver: false,
          errorYearTransferReceiver: null,
        });
      } else {
        const response = await CardService.findYearlyTransferReceiverAmount(
          token,
          year,
        );

        set({
          yearTransferReceiver: response,
          loadingYearTransferReceiver: false,
          errorYearTransferReceiver: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearTransferReceiver: false }),
        (message: any) => set({ errorYearTransferReceiver: message }),
        toast,
      );
    }
  },

  findMonthlyTransactionAmount: async (toast: any, year: number) => {
    set({
      loadingMonthTransaction: true,
      errorMonthTransaction: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthTransactionAmount(
          token,
          year,
        );

        set({
          monthTransactionAmount: response.data,
          loadingMonthTransaction: false,
          errorMonthTransaction: null,
        });
      } else {
        const response = await CardService.findMonthlyTransactionAmount(
          token,
          year,
        );

        set({
          monthTransactionAmount: response,
          loadingMonthTransaction: false,
          errorMonthTransaction: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingMonthTransaction: false }),
        (message: any) => set({ errorMonthTransaction: message }),
        toast,
      );
    }
  },

  findYearlyTransactionAmount: async (toast: any, year: number) => {
    set({
      loadingYearTransaction: true,
      errorYearTransaction: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findYearTransactionAmount(
          token,
          year,
        );

        set({
          yearTransactionAmount: response.data,
          loadingYearTransaction: false,
          errorYearTransaction: null,
        });
      } else {
        const response = await CardService.findYearlyTransactionAmount(
          token,
          year,
        );

        set({
          yearTransactionAmount: response,
          loadingYearTransaction: false,
          errorYearTransaction: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearTransaction: false }),
        (message: any) => set({ errorYearTransaction: message }),
        toast,
      );
    }
  },

  findMonthlyBalanceByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({ loadingMonthBalance: true, errorMonthBalance: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthBalanceByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          monthBalance: response.data,
          loadingMonthBalance: false,
          errorMonthBalance: null,
        });
      } else {
        const response = await CardService.findMonthlyBalanceByCard(
          token,
          year,
          card_number,
        );

        set({
          monthBalance: response,
          loadingMonthBalance: false,
          errorMonthBalance: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingMonthBalance: false }),
        (message: any) => set({ errorMonthBalance: message }),
        toast,
      );
    }
  },

  findYearlyBalanceByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({ loadingYearBalance: true, errorYearBalance: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findYearBalanceByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          yearBalance: response.data,
          loadingYearBalance: false,
          errorYearBalance: null,
        });
      } else {
        const response = await CardService.findYearlyBalanceByCard(
          token,
          year,
          card_number,
        );

        set({
          yearBalance: response,
          loadingYearBalance: false,
          errorYearBalance: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearBalance: false }),
        (message: any) => set({ errorYearBalance: message }),
        toast,
      );
    }
  },

  findMonthlyTopupAmountByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({
      loadingMonthTopupAmount: true,
      errorMonthTopupAmount: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthTopupAmountByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          monthTopupAmount: response.data,
          loadingMonthTopupAmount: false,
          errorMonthTopupAmount: null,
        });
      } else {
        const response = await CardService.findMonthlyTopupAmountByCard(
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
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingMonthTopupAmount: false }),
        (message: any) => set({ errorMonthTopupAmount: message }),
        toast,
      );
    }
  },

  findYearlyTopupAmountByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({
      loadingYearTopupAmount: true,
      errorYearTopupAmount: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findYearTopupAmountByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          yearTopupAmount: response.data,
          loadingYearTopupAmount: false,
          errorYearTopupAmount: null,
        });
      } else {
        const response = await CardService.findYearlyTopupAmountByCard(
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
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearTopupAmount: false }),
        (message: any) => set({ errorYearTopupAmount: message }),
        toast,
      );
    }
  },

  findMonthlyWithdrawAmountByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({
      loadingMonthWithdrawAmount: true,
      errorMonthWithdrawAmount: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthWithdrawAmountByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          monthWithdrawAmount: response.data,
          loadingMonthWithdrawAmount: false,
          errorMonthWithdrawAmount: null,
        });
      } else {
        const response = await CardService.findMonthlyWithdrawAmountByCard(
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
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingMonthWithdrawAmount: false }),
        (message: any) => set({ errorMonthWithdrawAmount: message }),
        toast,
      );
    }
  },

  findYearlyWithdrawAmountByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({
      loadingYearWithdrawAmount: true,
      errorYearWithdrawAmount: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findYearWithdrawAmountByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          yearWithdrawAmount: response.data,
          loadingYearWithdrawAmount: false,
          errorYearWithdrawAmount: null,
        });
      } else {
        const response = await CardService.findYearlyWithdrawAmountByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          yearWithdrawAmount: response,
          loadingYearWithdrawAmount: false,
          errorYearWithdrawAmount: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearWithdrawAmount: false }),
        (message: any) => set({ errorYearWithdrawAmount: message }),
        toast,
      );
    }
  },

  findMonthlyTransactionAmountByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({
      loadingMonthTransaction: true,
      errorMonthTransaction: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthTransactionAmountByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          monthTransactionAmount: response.data,
          loadingMonthTransaction: false,
          errorMonthTransaction: null,
        });
      } else {
        const response = await CardService.findMonthlyTransactionAmountByCard(
          token,
          year,
          card_number,
        );

        set({
          monthTransactionAmount: response,
          loadingMonthTransaction: false,
          errorMonthTransaction: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingMonthTransaction: false }),
        (message: any) => set({ errorMonthTransaction: message }),
        toast,
      );
    }
  },

  findYearlyTransactionAmountByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({
      loadingYearTransaction: true,
      errorYearTransaction: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findYearTransactionAmountByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          yearTransactionAmount: response.data,
          loadingYearTransaction: false,
          errorYearTransaction: null,
        });
      } else {
        const response = await CardService.findYearlyTransactionAmountByCard(
          token,
          year,
          card_number,
        );

        set({
          yearTransactionAmount: response,
          loadingYearTransaction: false,
          errorYearTransaction: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearTransaction: false }),
        (message: any) => set({ errorYearTransaction: message }),
        toast,
      );
    }
  },

  findMonthlyTransferSenderAmountByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({
      loadingMonthTransferSender: true,
      errorMonthTransferSender: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findMonthTransferSenderAmountByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          monthTransferSender: response.data,
          loadingMonthTransferSender: false,
          errorMonthTransferSender: null,
        });
      } else {
        const response =
          await CardService.findMonthlyTransferSenderAmountByCard(
            token,
            year,
            card_number,
          );

        set({
          monthTransferSender: response,
          loadingMonthTransferSender: false,
          errorMonthTransferSender: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingMonthTransferSender: false }),
        (message: any) => set({ errorMonthTransferSender: message }),
        toast,
      );
    }
  },

  findYearlyTransferSenderAmountByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({
      loadingYearTransferSender: true,
      errorYearTransferSender: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findYearTransferSenderAmountByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          yearTransferSender: response.data,
          loadingYearTransferSender: false,
          errorYearTransferSender: null,
        });
      } else {
        const response = await CardService.findYearlyTransferSenderAmountByCard(
          token,
          year,
          card_number,
        );

        set({
          yearTransferSender: response,
          loadingYearTransferSender: false,
          errorYearTransferSender: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearTransferSender: false }),
        (message: any) => set({ errorYearTransferSender: message }),
        toast,
      );
    }
  },

  findMonthlyTransferReceiverAmountByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({
      loadingMonthTransferReceiver: true,
      errorMonthTransferReceiver: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response =
          await CardCommand.findMonthTransferReceiverAmountByCard(
            token,
            year,
            card_number.toString(),
          );

        set({
          monthTransferReceiver: response.data,
          loadingMonthTransferReceiver: false,
          errorMonthTransferReceiver: null,
        });
      } else {
        const response =
          await CardService.findMonthlyTransferReceiverAmountByCard(
            token,
            year,
            card_number,
          );

        set({
          monthTransferReceiver: response,
          loadingMonthTransferReceiver: false,
          errorMonthTransferReceiver: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingMonthTransferReceiver: false }),
        (message: any) => set({ errorMonthTransferReceiver: message }),
        toast,
      );
    }
  },

  findYearlyTransferReceiverAmountByCard: async (
    toast: any,
    year: number,
    card_number: String,
  ) => {
    set({
      loadingYearTransferReceiver: true,
      errorYearTransferReceiver: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findYearTransferReceiverAmountByCard(
          token,
          year,
          card_number.toString(),
        );

        set({
          yearTransferReceiver: response.data,
          loadingYearTransferReceiver: false,
          errorYearTransferReceiver: null,
        });
      } else {
        const response =
          await CardService.findYearlyTransferReceiverAmountByCard(
            token,
            year,
            card_number,
          );
        console.log("hello", response);

        set({
          yearTransferReceiver: response,
          loadingYearTransferReceiver: false,
          errorYearTransferReceiver: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingYearTransferReceiver: false }),
        (message: any) => set({ errorYearTransferReceiver: message }),
        toast,
      );
    }
  },

  findAllCards: async (req: FindAllCard) => {
    set({ loadingGetCards: true, errorGetCards: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findAllCard(token, req);

        set({
          cards: response.data,
          pagination: {
            currentPage: response.pagination!.current_page,
            page_size: response.pagination!.page_size,
            totalItems: response.pagination!.total_records,
            totalPages: response.pagination!.total_pages,
          },
          loadingGetCards: false,
          errorGetCards: null,
        });
      } else {
        const response = await CardService.findAllCards(req, token);

        set({
          cards: response.data,
          pagination: {
            currentPage: response.pagination!.current_page,
            page_size: response.pagination!.page_size,
            totalItems: response.pagination!.total_records,
            totalPages: response.pagination!.total_pages,
          },
          loadingGetCards: false,
          errorGetCards: null,
        });
      }
    } catch (error) {
      handleApiError(
        error,
        () => set({ loadingGetCards: false }),
        (message: any) => set({ errorGetCards: message }),
        req.toast,
      );
    }
  },

  findByIdCard: async (req: FindByIdCard) => {
    set({ loadingGetCard: true, errorGetCard: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findByIdCard(token, req);

        set({
          card: response.data,
          loadingGetCard: false,
          errorGetCard: null,
        });
      } else {
        const response = await CardService.findByIdCard(req, token);

        set({
          card: response,
          loadingGetCard: false,
          errorGetCard: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetCard: false }),
        (message: any) => set({ errorGetCard: message }),
        req.toast,
      );
    }
  },

  findByUser: async (req: FindByUser) => {
    set({ loadingGetCardByUser: true, errorGetCardByUser: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findByUser(token, req);

        set({
          card: response.data,
          loadingGetCardByUser: false,
          errorGetCardByUser: null,
        });
      } else {
        const response = await CardService.findByUser(req, token);

        set({
          card: response,
          loadingGetCardByUser: false,
          errorGetCardByUser: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetCardByUser: false }),
        (message: any) => set({ errorGetCardByUser: message }),
        req.toast,
      );
    }
  },

  findByCardNumber: async (req: FindByCardNumber) => {
    set({ loadingGetCardByCardNumber: true, errorGetCardByCardNumber: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findByCardNumber(token, req);

        set({
          card: response.data,
          loadingGetCardByCardNumber: false,
          errorGetCardByCardNumber: null,
        });
      } else {
        const response = await CardService.findByCardNumber(req, token);

        set({
          card: response,
          loadingGetCardByCardNumber: false,
          errorGetCardByCardNumber: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetCardByCardNumber: false }),
        (message: any) => set({ errorGetCardByCardNumber: message }),
        req.toast,
      );
    }
  },

  findByActiveCard: async (req: FindAllCard) => {
    set({ loadingGetActiveCards: true, errorGetActiveCards: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.findActiveCard(token, req);

        set({
          cards: response.data,
          pagination: {
            currentPage: response.pagination!.current_page,
            page_size: response.pagination!.page_size,
            totalItems: response.pagination!.total_records,
            totalPages: response.pagination!.total_pages,
          },
          loadingGetActiveCards: false,
          errorGetActiveCards: null,
        });
      } else {
        const response = await CardService.findByActiveCard(req, token);

        set({
          cards: response.data,
          pagination: {
            currentPage: response.pagination!.current_page,
            page_size: response.pagination!.page_size,
            totalItems: response.pagination!.total_records,
            totalPages: response.pagination!.total_pages,
          },
          loadingGetActiveCards: false,
          errorGetActiveCards: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetActiveCards: false }),
        (message: any) => set({ errorGetActiveCards: message }),
        1,
      );
    }
  },

  createCard: async (req: CreateCard) => {
    set({ loadingCreateCard: true, errorCreateCard: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.createCard(token, req);

        handleMessageAction("card", "create");
        set({
          card: response.data,
          loadingCreateCard: false,
          errorCreateCard: null,
        });
      } else {
        const response = await CardService.createCard(req, token);

        handleMessageAction("card", "create");
        set({
          card: response,
          loadingCreateCard: false,
          errorCreateCard: null,
        });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingCreateCard: false }),
        (message: any) => set({ errorCreateCard: message }),
        req.toast,
      );

      return false;
    }
  },

  updateCard: async (req: UpdateCard) => {
    set({ loadingUpdateCard: true, errorUpdateCard: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommand.updateCard(token, req);

        handleMessageAction("card", "update");

        set({
          card: response.data,
          loadingUpdateCard: false,
          errorUpdateCard: null,
        });
      } else {
        const response = await CardService.updateCard(req, token);

        handleMessageAction("card", "update");

        set({
          card: response,
          loadingUpdateCard: false,
          errorUpdateCard: null,
        });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingUpdateCard: false }),
        (message: any) => set({ errorUpdateCard: message }),
        req.toast,
      );

      return false;
    }
  },

  trashedCard: async (req: FindByIdCard) => {
    set({ loadingTrashedCard: true, errorTrashedCard: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await CardCommand.trashedCard(token, req);

        set({ loadingTrashedCard: false, errorTrashedCard: null });

        handleMessageAction("card", "trashed");
      } else {
        await CardService.trashedCard(req, token);

        set({ loadingTrashedCard: false, errorTrashedCard: null });

        handleMessageAction("card", "trashed");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingTrashedCard: false }),
        (message: any) => set({ errorTrashedCard: message }),
        req.toast,
      );

      return false;
    }
  },
}));

export default useCardStore;
