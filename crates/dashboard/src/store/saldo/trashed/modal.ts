import { create } from "zustand";
import { ModalSaldoTrashedStore } from "@/types/state";

const useModalSaldoTrashed = create<ModalSaldoTrashedStore>((set) => ({
  isModalVisibleRestore: false,
  isModalVisibleDeletePermanent: false,
  isModalVisibleRestoreAll: false,
  isModalVisibleDeletePermanentAll: false,

  restoreSaldoId: null,
  deletePermanentSaldoId: null,

  showModalRestore: (id: number) =>
    set({ isModalVisibleRestore: true, restoreSaldoId: id }),
  hideModalRestore: () =>
    set({ isModalVisibleRestore: false, restoreSaldoId: null }),

  showModalDeletePermanent: (id: number) =>
    set({ isModalVisibleDeletePermanent: true, deletePermanentSaldoId: id }),
  hideModalDeletePermanent: () =>
    set({
      isModalVisibleDeletePermanent: false,
      deletePermanentSaldoId: null,
    }),

  showModalRestoreAll: () => set({ isModalVisibleRestoreAll: true }),
  hideModalRestoreAll: () => set({ isModalVisibleRestoreAll: false }),

  showModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: true }),
  hideModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: false }),
}));

export default useModalSaldoTrashed;
