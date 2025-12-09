import { create } from "zustand";
import { ModalTransferTrashedStore } from "@/types/state";

const useModalTransaferTrashed = create<ModalTransferTrashedStore>((set) => ({
  isModalVisibleRestore: false,
  isModalVisibleDeletePermanent: false,
  isModalVisibleRestoreAll: false,
  isModalVisibleDeletePermanentAll: false,

  restoreTransferId: null,
  deletePermanentTransferId: null,

  showModalRestore: (id: number) =>
    set({ isModalVisibleRestore: true, restoreTransferId: id }),
  hideModalRestore: () =>
    set({ isModalVisibleRestore: false, restoreTransferId: null }),

  showModalDeletePermanent: (id: number) =>
    set({
      isModalVisibleDeletePermanent: true,
      deletePermanentTransferId: id,
    }),
  hideModalDeletePermanent: () =>
    set({
      isModalVisibleDeletePermanent: false,
      deletePermanentTransferId: null,
    }),

  showModalRestoreAll: () => set({ isModalVisibleRestoreAll: true }),
  hideModalRestoreAll: () => set({ isModalVisibleRestoreAll: false }),

  showModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: true }),
  hideModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: false }),
}));

export default useModalTransaferTrashed;
