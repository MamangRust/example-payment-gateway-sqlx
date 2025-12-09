import { create } from "zustand";
import { ModalCardTrashedStore } from "@/types/state/card";

const useModalCardTrashed = create<ModalCardTrashedStore>((set) => ({
  isModalVisibleRestore: false,
  isModalVisibleDeletePermanent: false,
  isModalVisibleRestoreAll: false,
  isModalVisibleDeletePermanentAll: false,

  restoreCardId: null,
  deletePermanentCardId: null,

  showModalRestore: (id: number) =>
    set({ isModalVisibleRestore: true, restoreCardId: id }),
  hideModalRestore: () =>
    set({ isModalVisibleRestore: false, restoreCardId: null }),

  showModalDeletePermanent: (id: number) =>
    set({ isModalVisibleDeletePermanent: true, deletePermanentCardId: id }),
  hideModalDeletePermanent: () =>
    set({ isModalVisibleDeletePermanent: false, deletePermanentCardId: null }),

  showModalRestoreAll: () => set({ isModalVisibleRestoreAll: true }),
  hideModalRestoreAll: () => set({ isModalVisibleRestoreAll: false }),

  showModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: true }),
  hideModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: false }),
}));

export default useModalCardTrashed;
