import { create } from "zustand";
import { ModalTopupTrashedStore } from "@/types/state";

const useModalTopupTrashed = create<ModalTopupTrashedStore>((set) => ({
  isModalVisibleRestore: false,
  isModalVisibleDeletePermanent: false,
  isModalVisibleRestoreAll: false,
  isModalVisibleDeletePermanentAll: false,

  restoreTopupId: null,
  deletePermanentTopupId: null,

  showModalRestore: (id: number) =>
    set({ isModalVisibleRestore: true, restoreTopupId: id }),
  hideModalRestore: () =>
    set({ isModalVisibleRestore: false, restoreTopupId: null }),

  showModalDeletePermanent: (id: number) =>
    set({ isModalVisibleDeletePermanent: true, deletePermanentTopupId: id }),
  hideModalDeletePermanent: () =>
    set({
      isModalVisibleDeletePermanent: false,
      deletePermanentTopupId: null,
    }),

  showModalRestoreAll: () => set({ isModalVisibleRestoreAll: true }),
  hideModalRestoreAll: () => set({ isModalVisibleRestoreAll: false }),

  showModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: true }),
  hideModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: false }),
}));

export default useModalTopupTrashed;
