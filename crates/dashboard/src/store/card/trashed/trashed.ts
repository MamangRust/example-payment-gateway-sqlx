import { handleApiError } from "@/helpers/handleApi";
import { handleMessageAction } from "@/helpers/message";
import { getAccessToken } from "@/store/auth";
import {
  DeletePermanentCard,
  FindAllTrashedCard,
  RestoreTrashedCard,
} from "@/types/domain/request/card";
import { CardTrashedStore } from "@/types/state";
import { create } from "zustand";
import CardCommandTrashed from "@/services/ipc/card/card_trashed";
import CardServiceTrashed from "@/services/api/card/card_trashed";
import { isTauri } from "@tauri-apps/api/core";

const useCardTrashedStore = create<CardTrashedStore>((set, _get) => ({
  cards: null,

  pagination: {
    currentPage: 1,
    page_size: 10,
    totalItems: 0,
    totalPages: 0,
  },

  loadingGetCardsTrashed: false,
  loadingRestoreCardTrashed: false,
  loadingDeletePermanentCardTrashed: false,
  loadingRestoreAllCardTrashed: false,
  loadingDeletePermanentAllCardTrashed: false,

  errorGetCardsTrashed: null,
  errorRestoreCardTrashed: null,
  errorDeletePermanentCardTrashed: null,
  errorRestoreAllCardTrashed: null,
  errorDeletePermanentAllCardTrashed: null,

  setLoadingGetCardsTrashed: (value: boolean) =>
    set({ loadingGetCardsTrashed: value }),
  setLoadingRestoreCardTrashed: (value: boolean) =>
    set({ loadingRestoreCardTrashed: value }),
  setLoadingDeletePermanentCardTrashed: (value: boolean) =>
    set({ loadingDeletePermanentCardTrashed: value }),
  setLoadingRestoreAllCardTrashed: (value: boolean) =>
    set({ loadingRestoreAllCardTrashed: value }),
  setLoadingDeletePermanentAllCardTrashed: (value: boolean) =>
    set({ loadingDeletePermanentAllCardTrashed: value }),

  setErrorGetCardsTrashed: (value: string) =>
    set({ errorGetCardsTrashed: value }),
  setErrorRestoreCardTrashed: (value: string) =>
    set({ errorRestoreCardTrashed: value }),
  setErrorDeletePermanentCardTrashed: (value: string) =>
    set({ errorDeletePermanentCardTrashed: value }),
  setErrorRestoreAllCardTrashed: (value: string) =>
    set({ errorRestoreAllCardTrashed: value }),
  setErrorDeletePermanentAllCardTrashed: (value: string) =>
    set({ errorDeletePermanentAllCardTrashed: value }),

  findAllCardsTrashed: async (req: FindAllTrashedCard) => {
    set({ loadingGetCardsTrashed: true, errorGetCardsTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        const response = await CardCommandTrashed.findAllCardsTrashed(
          token,
          req,
        );

        set({
          cards: response.data,
          pagination: {
            currentPage: response.pagination!.current_page,
            page_size: response.pagination!.page_size,
            totalItems: response.pagination!.total_records,
            totalPages: response.pagination!.total_pages,
          },
          loadingGetCardsTrashed: false,
          errorGetCardsTrashed: null,
        });
      } else {
        const response = await CardServiceTrashed.findAllCardsTrashed(
          req,
          token,
        );

        set({
          cards: response.data,
          pagination: {
            currentPage: response.pagination!.current_page,
            page_size: response.pagination!.page_size,
            totalItems: response.pagination!.total_records,
            totalPages: response.pagination!.total_pages,
          },
          loadingGetCardsTrashed: false,
          errorGetCardsTrashed: null,
        });
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingGetCardsTrashed: false }),
        (message: any) => set({ errorGetCardsTrashed: message }),
        req.toast,
      );

      return false;
    }
  },

  restoreCardTrashed: async (req: RestoreTrashedCard) => {
    set({ loadingRestoreCardTrashed: true, errorRestoreCardTrashed: null });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await CardCommandTrashed.restoreCardTrashed(token, req);

        set({
          loadingRestoreCardTrashed: false,
          errorRestoreCardTrashed: null,
        });

        handleMessageAction("card", "restore");
      } else {
        await CardServiceTrashed.restoreCardTrashed(req, token);

        set({
          loadingRestoreCardTrashed: false,
          errorRestoreCardTrashed: null,
        });

        handleMessageAction("card", "restore");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingRestoreCardTrashed: false }),
        (message: any) => set({ errorRestoreCardTrashed: message }),
        req.toast,
      );

      return false;
    }
  },

  deletePermanentCard: async (req: DeletePermanentCard) => {
    set({
      loadingDeletePermanentCardTrashed: true,
      errorDeletePermanentCardTrashed: null,
    });
    try {
      const token = getAccessToken();

      if (isTauri()) {
        await CardCommandTrashed.deletePermanentCard(token, req);

        set({
          loadingDeletePermanentCardTrashed: false,
          errorDeletePermanentCardTrashed: null,
        });
        handleMessageAction("card", "deletePermanent");
      } else {
        await CardServiceTrashed.deletePermanentCard(req, token);

        set({
          loadingDeletePermanentCardTrashed: false,
          errorDeletePermanentCardTrashed: null,
        });
        handleMessageAction("card", "deletePermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentCardTrashed: false }),
        (message: any) => set({ errorDeletePermanentCardTrashed: message }),
        req.toast,
      );
      return false;
    }
  },
  restoreCardAllTrashed: async (toast: any) => {
    set({
      loadingRestoreAllCardTrashed: true,
      errorRestoreAllCardTrashed: null,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await CardCommandTrashed.restoreAllCardsTrashed(token);

        set({
          loadingDeletePermanentCardTrashed: false,
          errorDeletePermanentCardTrashed: null,
        });
        handleMessageAction("card", "restoreAll");
      } else {
        await CardServiceTrashed.restoreCardAllTrashed(token);

        set({
          loadingDeletePermanentCardTrashed: false,
          errorDeletePermanentCardTrashed: null,
        });
        handleMessageAction("card", "restoreAll");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentCardTrashed: false }),
        (message: any) => set({ errorDeletePermanentCardTrashed: message }),
        toast,
      );
      return false;
    }
  },

  deletePermanentAllCard: async (toast: any) => {
    set({
      loadingDeletePermanentAllCardTrashed: true,
      errorDeletePermanentAllCardTrashed: null,
    });

    try {
      const token = getAccessToken();

      if (isTauri()) {
        await CardCommandTrashed.deletePermanentAllCards(token);

        set({
          loadingDeletePermanentCardTrashed: false,
          errorDeletePermanentCardTrashed: null,
        });
        handleMessageAction("card", "deleteAllPermanent");
      } else {
        await CardServiceTrashed.deletePermanentAllCard(token);

        set({
          loadingDeletePermanentCardTrashed: false,
          errorDeletePermanentCardTrashed: null,
        });
        handleMessageAction("card", "deleteAllPermanent");
      }

      return true;
    } catch (err) {
      handleApiError(
        err,
        () => set({ loadingDeletePermanentCardTrashed: false }),
        (message: any) => set({ errorDeletePermanentCardTrashed: message }),
        toast,
      );

      return false;
    }
  },
}));

export default useCardTrashedStore;
