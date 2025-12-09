import { SaldoStore } from "@/types/state/saldo";
import { create } from "zustand";
import { getAccessToken } from "../auth";
import { handleApiError } from "@/helpers/handleApi";
import {
  CreateSaldo,
  UpdateSaldo,
  FindByIdSaldo,
  TrashedSaldo,
} from "@/types/domain/request";
import {
  FindAllSaldo,
  FindByCardNumberSaldo,
} from "@/types/domain/request/saldo";
import SaldoCommand from "@/services/ipc/saldo/saldo"
import SaldoService from "@/services/api/saldo/saldo";
import { handleMessageAction } from "@/helpers/message";
import { isTauri } from "@tauri-apps/api/core";

const useSaldoStore = create<SaldoStore>((set, _get) => ({
  saldos: null,
  saldo: null,

  monthTotalBalance: null,
  yearTotalBalance: null,

  monthBalance: null,
  yearBalance: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingMonthTotalBalance: false,
  loadingYearTotalBalance: false,
  loadingMonthBalance: false,
  loadingYearBalance: false,

  loadingGetSaldos: false,
  loadingGetSaldo: false,
  loadingGetActiveSaldo: false,
  loadingGetTrashedSaldo: false,
  loadingGetCardNumberSaldo: false,

  loadingCreateSaldo: false,
  loadingUpdateSaldo: false,
  loadingTrashedSaldo: false,
  loadingRestoreSaldo: false,
  loadingDeletePermanent: false,

  errorMonthTotalBalance: null,
  errorYearTotalBalance: null,
  errorMonthBalance: null,
  errorYearBalance: null,

  errorGetSaldos: null,
  errorGetSaldo: null,
  errorGetActiveSaldo: null,
  errorGetTrashedSaldo: null,
  errorGetCardNumberSaldo: null,

  errorCreateSaldo: null,
  errorUpdateSaldo: null,
  errorTrashedSaldo: null,
  errorRestoreSaldo: null,
  errorDeletePermanent: null,

  setLoadingMonthTotalBalance: (value) =>
    set({ loadingMonthTotalBalance: value }),
  setLoadingYearTotalBalance: (value) =>
    set({ loadingYearTotalBalance: value }),
  setLoadingMonthBalance: (value) => set({ loadingMonthBalance: value }),
  setLoadingYearBalance: (value) => set({ loadingYearBalance: value }),

  setLoadingGetSaldos: (value) => set({ loadingGetSaldos: value }),
  setLoadingGetSaldo: (value) => set({ loadingGetSaldo: value }),
  setLoadingGetActiveSaldo: (value) => set({ loadingGetActiveSaldo: value }),
  setLoadingGetTrashedSaldo: (value) => set({ loadingGetTrashedSaldo: value }),
  setLoadingGetCardNumberSaldo: (value) =>
    set({ loadingGetCardNumberSaldo: value }),

  setLoadingCreateSaldo: (value) => set({ loadingCreateSaldo: value }),
  setLoadingUpdateSaldo: (value) => set({ loadingUpdateSaldo: value }),
  setLoadingTrashedSaldo: (value) => set({ loadingTrashedSaldo: value }),

  setErrorMonthTotalBalance: (value) => set({ errorMonthTotalBalance: value }),
  setErrorYearTotalBalance: (value) => set({ errorYearTotalBalance: value }),
  setErrorMonthBalance: (value) => set({ errorMonthBalance: value }),
  setErrorYearBalance: (value) => set({ errorYearBalance: value }),

  setErrorGetSaldos: (value) => set({ errorGetSaldos: value }),
  setErrorGetSaldo: (value) => set({ errorGetSaldo: value }),
  setErrorGetActiveSaldo: (value) => set({ errorGetActiveSaldo: value }),
  setErrorGetTrashedSaldo: (value) => set({ errorGetTrashedSaldo: value }),
  setErrorGetCardNumberSaldo: (value) =>
    set({ errorGetCardNumberSaldo: value }),

  setErrorCreateSaldo: (value) => set({ errorCreateSaldo: value }),
  setErrorUpdateSaldo: (value) => set({ errorUpdateSaldo: value }),
  setErrorTrashedSaldo: (value) => set({ errorTrashedSaldo: value }),

  findMonthTotalBalance: async (toast: any, year: number, month: number) => {
    set({ loadingMonthTotalBalance: true, errorMonthTotalBalance: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await SaldoCommand.findMonthTotalBalance(
          token,
          year,
          month,
        );

        set({
          monthTotalBalance: response.data,
          loadingMonthTotalBalance: false,
          errorMonthTotalBalance: null,
        });

      } else {
        const response = await SaldoService.findMonthTotalBalance(
          token,
          year,
          month,
        );

        set({
          monthTotalBalance: response,
          loadingMonthTotalBalance: false,
          errorMonthTotalBalance: null,
        });

      }

    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingMonthTotalBalance: false }),
        (message: any) => set({ errorMonthTotalBalance: message }),
        toast,
      );
    }
  },

  findYearTotalBalance: async (toast: any, year: number) => {
    set({ loadingYearTotalBalance: true, errorYearTotalBalance: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await SaldoCommand.findYearTotalBalance(token, year);

        set({
          yearTotalBalance: response.data,
          loadingYearTotalBalance: false,
          errorYearTotalBalance: null,
        });
      } else {
        const response = await SaldoService.findYearTotalBalance(token, year);

        set({
          yearTotalBalance: response,
          loadingYearTotalBalance: false,
          errorYearTotalBalance: null,
        });
      }


    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearTotalBalance: false }),
        (message: any) => set({ errorYearTotalBalance: message }),
        toast,
      );
    }
  },

  findMonthBalance: async (toast: any, year: number) => {
    set({ loadingMonthBalance: true, errorMonthBalance: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await SaldoCommand.findMonthBalance(token, year);

        set({
          monthBalance: response.data,
          loadingMonthBalance: false,
          errorMonthBalance: null,
        });
      } else {
        const response = await SaldoService.findMonthBalance(token, year);

        set({
          monthBalance: response,
          loadingMonthBalance: false,
          errorMonthBalance: null,
        });
      }


    } catch (err) {
      handleApiError(
        err,
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
        const response = await SaldoCommand.findYearBalance(token, year);

        set({
          yearBalance: response.data,
          loadingYearBalance: false,
          errorYearBalance: null,
        });
      } else {
        const response = await SaldoService.findYearBalance(token, year);

        set({
          yearBalance: response,
          loadingYearBalance: false,
          errorYearBalance: null,
        });
      }


    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingYearBalance: false }),
        (message: any) => set({ errorYearBalance: message }),
        toast,
      );
    }
  },

  findAllSaldos: async (req: FindAllSaldo) => {
    set({ loadingGetSaldos: true, errorGetSaldos: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await SaldoCommand.findAllSaldos(token, req);



        set({
          saldos: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetSaldos: false,
          errorGetSaldos: null,
        });
      } else {
        const response = await SaldoService.findAllSaldos(token, req);

        set({
          saldos: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetSaldos: false,
          errorGetSaldos: null,
        });
      }


    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetSaldos: false }),
        (message: any) => set({ errorGetSaldos: message }),
        req.toast,
      );
    }
  },

  findByIdSaldo: async (req: FindByIdSaldo) => {
    set({ loadingGetSaldo: true, errorGetSaldo: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await SaldoCommand.findByIdSaldo(token, req);

        set({
          saldo: response.data,
          loadingGetSaldo: false,
          errorGetSaldo: null,
        });
      } else {
        const response = await SaldoService.findByIdSaldo(token, req);

        set({
          saldo: response,
          loadingGetSaldo: false,
          errorGetSaldo: null,
        });
      }


    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetSaldo: false }),
        (message: any) => set({ errorGetSaldo: message }),
        req.toast,
      );
    }
  },

  findByActiveSaldo: async (req: FindAllSaldo) => {
    set({ loadingGetActiveSaldo: true, errorGetActiveSaldo: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await SaldoCommand.findByActiveSaldo(token, req);

        set({
          saldos: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetSaldos: false,
          errorGetSaldos: null,
        });
      } else {
        const response = await SaldoService.findByActiveSaldo(token, req);

        set({
          saldos: response.data,
          pagination: {
            currentPage: response.pagination.current_page,
            page_size: response.pagination.page_size,
            totalItems: response.pagination.total_records,
            totalPages: response.pagination.total_pages,
          },
          loadingGetSaldos: false,
          errorGetSaldos: null,
        });
      }


    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetActiveSaldo: false }),
        (message: any) => set({ errorGetActiveSaldo: message }),
        null,
      );
    }
  },

  findByCardNumberSaldo: async (req: FindByCardNumberSaldo) => {
    set({ loadingGetCardNumberSaldo: true, errorGetCardNumberSaldo: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {

      } else {
        const response = await SaldoService.findByCardNumberSaldo(token, req);

        set({
          saldo: response,
          loadingGetCardNumberSaldo: false,
          errorGetCardNumberSaldo: null,
        });
      }


    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetCardNumberSaldo: false }),
        (message: any) => set({ errorGetCardNumberSaldo: message }),
        req.toast,
      );
    }
  },

  createSaldo: async (req: CreateSaldo) => {
    set({ loadingCreateSaldo: true, errorCreateSaldo: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await SaldoCommand.createSaldo(token, req);

        handleMessageAction("saldo", "create");

        set({ loadingCreateSaldo: false, errorCreateSaldo: null });
      } else {
        await SaldoService.createSaldo(token, req);

        handleMessageAction("saldo", "create");

        set({ loadingCreateSaldo: false, errorCreateSaldo: null });
      }


      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingCreateSaldo: false }),
        (message: any) => set({ errorCreateSaldo: message }),
        req.toast,
      );

      return false;
    }
  },

  updateSaldo: async (req: UpdateSaldo) => {
    set({ loadingUpdateSaldo: true, errorUpdateSaldo: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await SaldoCommand.updateSaldo(token, req);

        handleMessageAction("saldo", "update");

        set({ loadingUpdateSaldo: false, errorUpdateSaldo: null });
      } else {
        await SaldoService.updateSaldo(token, req);

        handleMessageAction("saldo", "update");

        set({ loadingUpdateSaldo: false, errorUpdateSaldo: null });
      }


      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingUpdateSaldo: false }),
        (message: any) => set({ errorUpdateSaldo: message }),
        req.toast,
      );
      return false;
    }
  },

  trashedSaldo: async (req: TrashedSaldo) => {
    set({ loadingTrashedSaldo: true, errorTrashedSaldo: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await SaldoCommand.trashedSaldo(token, req);

        handleMessageAction("saldo", "trashed");

        set({ loadingTrashedSaldo: false, errorTrashedSaldo: null });
      } else {
        await SaldoService.trashedSaldo(token, req);

        handleMessageAction("saldo", "trashed");

        set({ loadingTrashedSaldo: false, errorTrashedSaldo: null });
      }



      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingTrashedSaldo: false }),
        (message: any) => set({ errorTrashedSaldo: message }),
        req.toast,
      );

      return false;
    }
  },
}));

export default useSaldoStore;
