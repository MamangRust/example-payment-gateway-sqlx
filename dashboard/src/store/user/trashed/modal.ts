import { create } from "zustand";
import { ModalUserTrashedStore } from "@/types/state";

const useModalUserTrashed = create<ModalUserTrashedStore>((set) => ({
  isModalVisibleRestore: false,
  isModalVisibleDeletePermanent: false,
  isModalVisibleRestoreAll: false,
  isModalVisibleDeletePermanentAll: false,

  restoreUserId: null,
  deletePermanentUserId: null,

  showModalRestore: (id: number) =>
    set({ isModalVisibleRestore: true, restoreUserId: id }),
  hideModalRestore: () =>
    set({ isModalVisibleRestore: false, restoreUserId: null }),

  showModalDeletePermanent: (id: number) =>
    set({
      isModalVisibleDeletePermanent: true,
      deletePermanentUserId: id,
    }),
  hideModalDeletePermanent: () =>
    set({
      isModalVisibleDeletePermanent: false,
      deletePermanentUserId: null,
    }),

  showModalRestoreAll: () => set({ isModalVisibleRestoreAll: true }),
  hideModalRestoreAll: () => set({ isModalVisibleRestoreAll: false }),

  showModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: true }),
  hideModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: false }),
}));

export default useModalUserTrashed;
