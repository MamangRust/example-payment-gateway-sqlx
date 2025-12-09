import { create } from "zustand";
import { ModalWithdrawTrashedStore } from "@/types/state";

const useModalWithdrawTrashed = create<ModalWithdrawTrashedStore>((set) => ({
  isModalVisibleRestore: false,
  isModalVisibleDeletePermanent: false,
  isModalVisibleRestoreAll: false,
  isModalVisibleDeletePermanentAll: false,

  restoreWithdrawId: null,
  deletePermanentWithdrawId: null,

  showModalRestore: (id: number) =>
    set({ isModalVisibleRestore: true, restoreWithdrawId: id }),
  hideModalRestore: () =>
    set({ isModalVisibleRestore: false, restoreWithdrawId: null }),

  showModalDeletePermanent: (id: number) =>
    set({
      isModalVisibleDeletePermanent: true,
      deletePermanentWithdrawId: id,
    }),
  hideModalDeletePermanent: () =>
    set({
      isModalVisibleDeletePermanent: false,
      deletePermanentWithdrawId: null,
    }),

  showModalRestoreAll: () => set({ isModalVisibleRestoreAll: true }),
  hideModalRestoreAll: () => set({ isModalVisibleRestoreAll: false }),

  showModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: true }),
  hideModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: false }),
}));

export default useModalWithdrawTrashed;
