import { create } from "zustand";
import { ModalTransactionTrashedStore } from "@/types/state";

const useModalTransactionTrashed = create<ModalTransactionTrashedStore>(
  (set) => ({
    isModalVisibleRestore: false,
    isModalVisibleDeletePermanent: false,
    isModalVisibleRestoreAll: false,
    isModalVisibleDeletePermanentAll: false,

    restoreTransactionId: null,
    deletePermanentTransactionId: null,

    showModalRestore: (id: number) =>
      set({ isModalVisibleRestore: true, restoreTransactionId: id }),
    hideModalRestore: () =>
      set({ isModalVisibleRestore: false, restoreTransactionId: null }),

    showModalDeletePermanent: (id: number) =>
      set({
        isModalVisibleDeletePermanent: true,
        deletePermanentTransactionId: id,
      }),
    hideModalDeletePermanent: () =>
      set({
        isModalVisibleDeletePermanent: false,
        deletePermanentTransactionId: null,
      }),

    showModalRestoreAll: () => set({ isModalVisibleRestoreAll: true }),
    hideModalRestoreAll: () => set({ isModalVisibleRestoreAll: false }),

    showModalDeletePermanentAll: () =>
      set({ isModalVisibleDeletePermanentAll: true }),
    hideModalDeletePermanentAll: () =>
      set({ isModalVisibleDeletePermanentAll: false }),
  }),
);

export default useModalTransactionTrashed;
