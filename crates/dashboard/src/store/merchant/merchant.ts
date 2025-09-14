import { create } from "zustand";
import { handleApiError } from "@/helpers/handleApi";
import { MerchantStore } from "@/types/state/merchant/merchant";
import {
  CreateMerchant,
  FindAllMerchant,
  FindAllMerchantTransaction,
  FindAllTransactionByApiKey,
  findByApiKeyMerchant,
  FindByIdMerchant,
  FindMerchantUser,
  FindTrashedMerchant,
  UpdateMerchant,
} from "@/types/domain/request";
import MerchantService from "@/services/api/merchant/merchant";
import MerchantCommand from "@/services/ipc/merchant/merchant";
import { getAccessToken } from "../auth";
import { handleMessageAction } from "@/helpers/message";
import { isTauri } from "@tauri-apps/api/core";

const useMerchantStore = create<MerchantStore>((set, _get) => ({
  merchants: null,
  merchant: null,

  transactions: null,

  monthPaymentMethod: null,
  yearPaymentMethod: null,

  monthAmount: null,
  yearAmount: null,

  monthTotalAmount: null,
  yearTotalAmount: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  paginationTransaction: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingMonthPaymentMethod: false,
  loadingYearPaymentMethod: false,
  loadingMonthAmount: false,
  loadingYearAmount: false,
  loadingMonthTotalAmount: false,
  loadingYearTotalAmount: false,

  loadingGetMerchants: false,
  loadingGetTransactions: false,
  loadingGetMerchant: false,
  loadingGetApiKey: false,
  loadingGetMerchantUser: false,
  loadingGetActiveMerchant: false,
  loadingGetTrashedMerchant: false,
  loadingCreateMerchant: false,
  loadingUpdateMerchant: false,
  loadingTrashedMerchant: false,
  loadingRestoreMerchant: false,
  loadingDeletePermanentMerchant: false,

  errorMonthPaymentMethod: null,
  errorYearPaymentMethod: null,
  errorMonthAmount: null,
  errorYearAmount: null,
  errorMonthTotalAmount: null,
  errorYearTotalAmount: null,

  errorGetMerchants: null,
  errorGetTransactions: null,
  errorGetMerchant: null,
  errorGetApiKey: null,
  errorGetMerchantUser: null,
  errorGetActiveMerchant: null,
  errorGetTrashedMerchant: null,
  errorCreateMerchant: null,
  errorUpdateMerchant: null,
  errorTrashedMerchant: null,

  setLoadingMonthPaymentMethod: (value: boolean) =>
    set({ loadingMonthPaymentMethod: value }),
  setLoadingYearPaymentMethod: (value: boolean) =>
    set({ loadingYearPaymentMethod: value }),
  setLoadingMonthAmount: (value: boolean) => set({ loadingMonthAmount: value }),
  setLoadingYearAmount: (value: boolean) => set({ loadingYearAmount: value }),
  setLoadingMonthTotalAmount: (value: boolean) =>
    set({ loadingMonthTotalAmount: value }),
  setLoadingYearTotalAmount: (value: boolean) =>
    set({ loadingYearTotalAmount: value }),

  setLoadingGetMerchants: (value) => set({ loadingGetMerchants: value }),
  setLoadingGetTransactions: (value) => set({ loadingGetTransactions: value }),
  setLoadingGetMerchant: (value) => set({ loadingGetMerchant: value }),
  setLoadingGetApiKey: (value) => set({ loadingGetApiKey: value }),
  setLoadingGetMerchantUser: (value) => set({ loadingGetMerchantUser: value }),
  setLoadingGetActiveMerchant: (value) =>
    set({ loadingGetActiveMerchant: value }),
  setLoadingGetTrashedMerchant: (value) =>
    set({ loadingGetTrashedMerchant: value }),
  setLoadingCreateMerchant: (value) => set({ loadingCreateMerchant: value }),
  setLoadingUpdateMerchant: (value) => set({ loadingUpdateMerchant: value }),
  setLoadingTrashedMerchant: (value) => set({ loadingTrashedMerchant: value }),

  setErrorMonthPaymentMethod: (value) =>
    set({ errorMonthPaymentMethod: value }),
  setErrorYearPaymentMethod: (value) => set({ errorYearPaymentMethod: value }),
  setErrorMonthAmount: (value) => set({ errorMonthAmount: value }),
  setErrorYearAmount: (value) => set({ errorYearAmount: value }),

  setErrorMonthTotalAmount: (value) => set({ errorMonthTotalAmount: value }),
  setErrorYearTotalAmount: (value) => set({ errorYearTotalAmount: value }),

  setErrorGetMerchants: (value) => set({ errorGetMerchants: value }),
  setErrorGetTransactions: (value) => set({ errorGetTransactions: value }),
  setErrorGetMerchant: (value) => set({ errorGetMerchant: value }),
  setErrorGetApiKey: (value) => set({ errorGetApiKey: value }),
  setErrorGetMerchantUser: (value) => set({ errorGetMerchantUser: value }),
  setErrorGetActiveMerchant: (value) => set({ errorGetActiveMerchant: value }),
  setErrorGetTrashedMerchant: (value) =>
    set({ errorGetTrashedMerchant: value }),
  setErrorCreateMerchant: (value) => set({ errorCreateMerchant: value }),
  setErrorUpdateMerchant: (value) => set({ errorUpdateMerchant: value }),
  setErrorTrashedMerchant: (value) => set({ errorTrashedMerchant: value }),

  findMonthPaymentMethod: async (toast: any, year: number) => {
    set({ loadingMonthPaymentMethod: true, errorMonthPaymentMethod: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMonthPaymentMethod(
          token,
          year,
        );

        set({
          monthPaymentMethod: response.data,
          loadingMonthPaymentMethod: false,
          errorMonthPaymentMethod: null,
        });
      } else {
        const response = await MerchantService.findMonthPaymentMethod(
          token,
          year,
        );

        set({
          monthPaymentMethod: response,
          loadingMonthPaymentMethod: false,
          errorMonthPaymentMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthPaymentMethod: false }),
        (message: any) => set({ errorMonthPaymentMethod: message }),
        toast,
      );
    }
  },

  findYearPaymentMethod: async (toast: any, year: number) => {
    set({ loadingYearPaymentMethod: true, errorYearPaymentMethod: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findYearPaymentMethod(
          token,
          year,
        );

        set({
          yearPaymentMethod: response.data,
          loadingYearPaymentMethod: false,
          errorYearPaymentMethod: null,
        });
      } else {
        const response = await MerchantService.findYearPaymentMethod(
          token,
          year,
        );

        set({
          yearPaymentMethod: response,
          loadingYearPaymentMethod: false,
          errorYearPaymentMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearPaymentMethod: false }),
        (message: any) => set({ errorYearPaymentMethod: message }),
        toast,
      );
    }
  },

  findMonthAmount: async (toast: any, year: number) => {
    set({ loadingMonthAmount: true, errorMonthAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMonthAmount(token, year);

        set({
          monthAmount: response.data,
          loadingMonthAmount: false,
          errorMonthAmount: null,
        });
      } else {
        const response = await MerchantService.findMonthAmount(token, year);

        set({
          monthAmount: response,
          loadingMonthAmount: false,
          errorMonthAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthAmount: false }),
        (message: any) => set({ errorMonthAmount: message }),
        toast,
      );
    }
  },

  findYearAmount: async (toast: any, year: number) => {
    set({ loadingYearAmount: true, errorYearAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findYearAmount(token, year);

        set({
          yearAmount: response.data,
          loadingYearAmount: false,
          errorYearAmount: null,
        });
      } else {
        const response = await MerchantService.findYearAmount(token, year);

        set({
          yearAmount: response,
          loadingYearAmount: false,
          errorYearAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearAmount: false }),
        (message: any) => set({ errorYearAmount: message }),
        toast,
      );
    }
  },

  findMonthTotalAmount: async (toast: any, year: number, month: number) => {
    set({ loadingMonthTotalAmount: true, errorMonthTotalAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMonthTotalAmount(
          token,
          year,
          month,
        );

        set({
          monthTotalAmount: response.data,
          loadingMonthTotalAmount: false,
          errorMonthTotalAmount: null,
        });
      } else {
        const response = await MerchantService.findMonthTotalAmount(
          token,
          year,
          month,
        );

        set({
          monthTotalAmount: response,
          loadingMonthTotalAmount: false,
          errorMonthTotalAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTotalAmount: false }),
        (message: any) => set({ errorMonthTotalAmount: message }),
        toast,
      );
    }
  },

  findYearTotalAmount: async (toast: any, year: number) => {
    set({ loadingYearAmount: true, errorYearAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findYearTotalAmount(token, year);

        set({
          yearTotalAmount: response.data,
          loadingYearAmount: false,
          errorYearAmount: null,
        });
      } else {
        const response = await MerchantService.findYearTotalAmount(token, year);

        set({
          yearTotalAmount: response,
          loadingYearAmount: false,
          errorYearAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearAmount: false }),
        (message: any) => set({ errorYearAmount: message }),
        toast,
      );
    }
  },

  findMonthPaymentMethodByMerchant: async (
    toast: any,
    year: number,
    merchant_id: number,
  ) => {
    set({ loadingMonthPaymentMethod: true, errorMonthPaymentMethod: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMonthPaymentMethodByMerchant(
          token,
          year,
          merchant_id,
        );

        set({
          monthPaymentMethod: response.data,
          loadingMonthPaymentMethod: false,
          errorMonthPaymentMethod: null,
        });
      } else {
        const response = await MerchantService.findMonthPaymentMethodByMerchant(
          token,
          year,
          merchant_id,
        );

        set({
          monthPaymentMethod: response,
          loadingMonthPaymentMethod: false,
          errorMonthPaymentMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthPaymentMethod: false }),
        (message: any) => set({ errorMonthPaymentMethod: message }),
        toast,
      );
    }
  },

  findYearPaymentMethodByMerchant: async (
    toast: any,
    year: number,
    merchant_id: number,
  ) => {
    set({ loadingYearPaymentMethod: true, errorYearPaymentMethod: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findYearPaymentMethodByMerchant(
          token,
          year,
          merchant_id,
        );

        set({
          yearPaymentMethod: response.data,
          loadingYearPaymentMethod: false,
          errorYearPaymentMethod: null,
        });
      } else {
        const response = await MerchantService.findYearPaymentMethodByMerchant(
          token,
          year,
          merchant_id,
        );

        set({
          yearPaymentMethod: response,
          loadingYearPaymentMethod: false,
          errorYearPaymentMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearPaymentMethod: false }),
        (message: any) => set({ errorYearPaymentMethod: message }),
        toast,
      );
    }
  },

  findMonthAmountByMerchant: async (
    toast: any,
    year: number,
    merchant_id: number,
  ) => {
    set({ loadingMonthAmount: true, errorMonthAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMonthAmountByMerchant(
          token,
          year,
          merchant_id,
        );

        set({
          monthAmount: response.data,
          loadingMonthAmount: false,
          errorMonthAmount: null,
        });
      } else {
        const response = await MerchantService.findMonthAmountByMerchant(
          token,
          year,
          merchant_id,
        );

        set({
          monthAmount: response,
          loadingMonthAmount: false,
          errorMonthAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthAmount: false }),
        (message: any) => set({ errorMonthAmount: message }),
        toast,
      );
    }
  },

  findYearAmountByMerchant: async (
    toast: any,
    year: number,
    merchant_id: number,
  ) => {
    set({ loadingYearAmount: true, errorYearAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findYearAmountByMerchant(
          token,
          year,
          merchant_id,
        );

        set({
          yearAmount: response.data,
          loadingYearAmount: false,
          errorYearAmount: null,
        });
      } else {
        const response = await MerchantService.findYearAmountByMerchant(
          token,
          year,
          merchant_id,
        );

        set({
          yearAmount: response,
          loadingYearAmount: false,
          errorYearAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearAmount: false }),
        (message: any) => set({ errorYearAmount: message }),
        toast,
      );
    }
  },

  findMonthTotalAmountByMerchant: async (
    toast: any,
    year: number,
    month: number,
    merchant_id: number,
  ) => {
    set({ loadingMonthTotalAmount: true, errorMonthTotalAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMonthTotalAmountByMerchant(
          token,
          year,
          month,
          merchant_id,
        );

        set({
          monthTotalAmount: response.data,
          loadingMonthTotalAmount: false,
          errorMonthTotalAmount: null,
        });
      } else {
        const response = await MerchantService.findMonthTotalAmountByMerchant(
          token,
          year,
          month,
          merchant_id,
        );

        set({
          monthTotalAmount: response,
          loadingMonthTotalAmount: false,
          errorMonthTotalAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTotalAmount: false }),
        (message: any) => set({ errorMonthTotalAmount: message }),
        toast,
      );
    }
  },

  findYearTotalAmountByMerchant: async (
    toast: any,
    year: number,
    merchant_id: number,
  ) => {
    set({ loadingYearTotalAmount: true, errorYearTotalAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findYearTotalAmountByMerchant(
          token,
          year,
          merchant_id,
        );

        set({
          yearTotalAmount: response.data,
          loadingYearTotalAmount: false,
          errorYearTotalAmount: null,
        });
      } else {
        const response = await MerchantService.findYearTotalAmountByMerchant(
          token,
          year,
          merchant_id,
        );

        set({
          yearTotalAmount: response,
          loadingYearTotalAmount: false,
          errorYearTotalAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTotalAmount: false }),
        (message: any) => set({ errorYearTotalAmount: message }),
        toast,
      );
    }
  },

  findMonthPaymentMethodByApiKey: async (
    toast: any,
    year: number,
    api_key: string,
  ) => {
    set({ loadingMonthPaymentMethod: true, errorMonthPaymentMethod: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMonthPaymentMethodByApiKey(
          token,
          year,
          api_key,
        );

        set({
          monthPaymentMethod: response.data,
          loadingMonthPaymentMethod: false,
          errorMonthPaymentMethod: null,
        });
      } else {
        const response = await MerchantService.findMonthPaymentMethodByApiKey(
          token,
          year,
          api_key,
        );

        set({
          monthPaymentMethod: response,
          loadingMonthPaymentMethod: false,
          errorMonthPaymentMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthPaymentMethod: false }),
        (message: any) => set({ errorMonthPaymentMethod: message }),
        toast,
      );
    }
  },

  findYearPaymentMethodByApiKey: async (
    toast: any,
    year: number,
    api_key: string,
  ) => {
    set({ loadingYearPaymentMethod: true, errorYearPaymentMethod: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findYearPaymentMethodByApiKey(
          token,
          year,
          api_key,
        );

        set({
          yearPaymentMethod: response.data,
          loadingYearPaymentMethod: false,
          errorYearPaymentMethod: null,
        });
      } else {
        const response = await MerchantService.findYearPaymentMethodByApiKey(
          token,
          year,
          api_key,
        );

        set({
          yearPaymentMethod: response,
          loadingYearPaymentMethod: false,
          errorYearPaymentMethod: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearPaymentMethod: false }),
        (message: any) => set({ errorYearPaymentMethod: message }),
        toast,
      );
    }
  },

  findMonthAmountByApiKey: async (
    toast: any,
    year: number,
    api_key: string,
  ) => {
    set({ loadingMonthAmount: true, errorMonthAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMonthAmountByApiKey(
          token,
          year,
          api_key,
        );

        set({
          monthAmount: response.data,
          loadingMonthAmount: false,
          errorMonthAmount: null,
        });
      } else {
        const response = await MerchantService.findMonthAmountByApiKey(
          token,
          year,
          api_key,
        );

        set({
          monthAmount: response,
          loadingMonthAmount: false,
          errorMonthAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthAmount: false }),
        (message: any) => set({ errorMonthAmount: message }),
        toast,
      );
    }
  },

  findYearAmountByApiKey: async (toast: any, year: number, api_key: string) => {
    set({ loadingYearAmount: true, errorYearAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findYearAmountByApiKey(
          token,
          year,
          api_key,
        );

        set({
          yearAmount: response.data,
          loadingYearAmount: false,
          errorYearAmount: null,
        });
      } else {
        const response = await MerchantService.findYearAmountByApiKey(
          token,
          year,
          api_key,
        );

        set({
          yearAmount: response,
          loadingYearAmount: false,
          errorYearAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearAmount: false }),
        (message: any) => set({ errorYearAmount: message }),
        toast,
      );
    }
  },

  findMonthTotalAmountByApiKey: async (
    toast: any,
    year: number,
    month: number,
    api_key: string,
  ) => {
    set({ loadingMonthTotalAmount: true, errorMonthTotalAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMonthTotalAmountByApiKey(
          token,
          year,
          month,
          api_key,
        );

        set({
          monthTotalAmount: response.data,
          loadingMonthTotalAmount: false,
          errorMonthTotalAmount: null,
        });
      } else {
        const response = await MerchantService.findMonthTotalAmountByApiKey(
          token,
          year,
          month,
          api_key,
        );

        set({
          monthTotalAmount: response,
          loadingMonthTotalAmount: false,
          errorMonthTotalAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTotalAmount: false }),
        (message: any) => set({ errorMonthTotalAmount: message }),
        toast,
      );
    }
  },

  findYearTotalAmountByApiKey: async (
    toast: any,
    year: number,
    api_key: string,
  ) => {
    set({ loadingYearTotalAmount: true, errorYearTotalAmount: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findYearTotalAmountByApiKey(
          token,
          year,
          api_key,
        );

        set({
          yearTotalAmount: response.data,
          loadingYearTotalAmount: false,
          errorYearTotalAmount: null,
        });
      } else {
        const response = await MerchantService.findYearTotalAmountByApiKey(
          token,
          year,
          api_key,
        );

        set({
          yearTotalAmount: response,
          loadingYearTotalAmount: false,
          errorYearTotalAmount: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTotalAmount: false }),
        (message: any) => set({ errorYearTotalAmount: message }),
        toast,
      );
    }
  },

  findAllMerchants: async (req: FindAllMerchant) => {
    set({ loadingGetMerchants: true, errorGetMerchants: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findAllMerchants(token, req);

        set({
          merchants: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetMerchants: false,
          errorGetMerchants: null,
        });
      } else {
        const response = await MerchantService.findAllMerchants(req, token);

        set({
          merchants: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetMerchants: false,
          errorGetMerchants: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetMerchants: false }),
        (message: any) => set({ errorGetMerchants: message }),
        req.toast,
      );
    }
  },

  findAllTransaction: async (req: FindAllMerchant) => {
    set({ loadingGetTransactions: true, errorGetTransactions: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findAllTransactions(token, req);

        set({
          transactions: response.data,
          paginationTransaction: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTransactions: false,
          errorGetTransactions: null,
        });
      } else {
        const response = await MerchantService.findAllTransaction(req, token);

        set({
          transactions: response.data,
          paginationTransaction: {
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

  findAllTransactionByMerchant: async (req: FindAllMerchantTransaction) => {
    set({ loadingGetTransactions: true, errorGetTransactions: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findAllTransactionByMerchant(
          token,
          req,
        );

        set({
          transactions: response.data,
          paginationTransaction: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTransactions: false,
          errorGetTransactions: null,
        });
      } else {
        const response = await MerchantService.findAllTransactionByMerchant(
          req,
          token,
        );

        set({
          transactions: response.data,
          paginationTransaction: {
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

  findAllTransactionByApiKey: async (req: FindAllTransactionByApiKey) => {
    set({ loadingGetTransactions: true, errorGetTransactions: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findAllTransactionByApiKey(
          token,
          req,
        );

        set({
          transactions: response.data,
          paginationTransaction: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTransactions: false,
          errorGetTransactions: null,
        });
      } else {
        const response = await MerchantService.findAllTransactionByApiKey(
          req,
          token,
        );

        set({
          transactions: response.data,
          paginationTransaction: {
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

  findById: async (req: FindByIdMerchant) => {
    set({ loadingGetMerchant: true, errorGetMerchant: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMerchantById(token, req);

        set({
          merchant: response.data,
          loadingGetMerchant: false,
          errorGetMerchant: null,
        });
      } else {
        const response = await MerchantService.findById(req, token);

        set({
          merchant: response,
          loadingGetMerchant: false,
          errorGetMerchant: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetMerchant: false }),
        (message: any) => set({ errorGetMerchant: message }),
        req.toast,
      );
    }
  },

  findByApiKey: async (req: findByApiKeyMerchant) => {
    set({ loadingGetApiKey: true, errorGetApiKey: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMerchantByApiKey(token, req);

        set({
          merchant: response.data,
          loadingGetApiKey: false,
          errorGetApiKey: null,
        });
      } else {
        const response = await MerchantService.findByApiKey(req, token);

        set({
          merchant: response,
          loadingGetApiKey: false,
          errorGetApiKey: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetApiKey: false }),
        (message: any) => set({ errorGetApiKey: message }),
        req.toast,
      );
    }
  },

  findByMerchantUser: async (req: FindMerchantUser) => {
    set({ loadingGetMerchantUser: true, errorGetMerchantUser: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findMerchantByUser(token, req);

        set({
          merchants: response.data,
          loadingGetMerchantUser: false,
          errorGetMerchantUser: null,
        });
      } else {
        const response = await MerchantService.findByMerchantUser(req, token);

        set({
          merchants: response,
          loadingGetMerchantUser: false,
          errorGetMerchantUser: null,
        });
      }
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetMerchantUser: false }),
        (message: any) => set({ errorGetMerchantUser: message }),
        req.toast,
      );
    }
  },

  findByActive: async (req: FindAllMerchant) => {
    set({ loadingGetActiveMerchant: true, errorGetActiveMerchant: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await MerchantCommand.findActiveMerchant(token, req);

        set({
          merchants: response.data,
          paginationTransaction: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetTransactions: false,
          errorGetTransactions: null,
        });
      } else {
        const response = await MerchantService.findByActive(req, token);

        set({
          merchants: response.data,
          paginationTransaction: {
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
        () => set({ loadingGetActiveMerchant: false }),
        () => set({ errorGetActiveMerchant: null }),
        null,
      );
    }
  },

  createMerchant: async (req: CreateMerchant) => {
    set({ loadingCreateMerchant: true, errorCreateMerchant: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await MerchantCommand.createMerchant(token, req);

        handleMessageAction("merchant", "create");

        set({ loadingCreateMerchant: false, errorCreateMerchant: null });
      } else {
        await MerchantService.createMerchant(req, token);

        handleMessageAction("merchant", "create");

        set({ loadingCreateMerchant: false, errorCreateMerchant: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingCreateMerchant: false }),
        (message: any) => set({ errorCreateMerchant: message }),
        req.toast,
      );

      return false;
    }
  },

  updateMerchant: async (req: UpdateMerchant) => {
    set({ loadingUpdateMerchant: true, errorUpdateMerchant: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await MerchantCommand.updateMerchant(token, req);

        handleMessageAction("merchant", "update");

        set({ loadingUpdateMerchant: false, errorUpdateMerchant: null });
      } else {
        await MerchantService.updateMerchant(req, token);

        handleMessageAction("merchant", "update");

        set({ loadingUpdateMerchant: false, errorUpdateMerchant: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingUpdateMerchant: false }),
        (message: any) => set({ errorUpdateMerchant: message }),
        req.toast,
      );

      return false;
    }
  },
  trashedMerchant: async (req: FindTrashedMerchant) => {
    set({ loadingTrashedMerchant: true, errorTrashedMerchant: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await MerchantCommand.trashedMerchant(token, req);

        handleMessageAction("merchant", "trashed");

        set({ loadingTrashedMerchant: false, errorTrashedMerchant: null });
      } else {
        await MerchantService.trashedMerchant(req, token);

        handleMessageAction("merchant", "trashed");

        set({ loadingTrashedMerchant: false, errorTrashedMerchant: null });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingTrashedMerchant: false }),
        (message: any) => set({ errorTrashedMerchant: message }),
        req.toast,
      );
      return false;
    }
  },
}));

export default useMerchantStore;
