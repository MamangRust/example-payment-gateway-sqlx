import { create } from "zustand";
import { ModalMerchantTrashedStore } from "@/types/state";

const useModalMerchantTrashed = create<ModalMerchantTrashedStore>((set) => ({
  isModalVisibleRestore: false,
  isModalVisibleDeletePermanent: false,
  isModalVisibleRestoreAll: false,
  isModalVisibleDeletePermanentAll: false,

  restoreMerchantId: null,
  deletePermanentMerchantId: null,

  showModalRestore: (id: number) =>
    set({ isModalVisibleRestore: true, restoreMerchantId: id }),
  hideModalRestore: () =>
    set({ isModalVisibleRestore: false, restoreMerchantId: null }),

  showModalDeletePermanent: (id: number) =>
    set({ isModalVisibleDeletePermanent: true, deletePermanentMerchantId: id }),
  hideModalDeletePermanent: () =>
    set({
      isModalVisibleDeletePermanent: false,
      deletePermanentMerchantId: null,
    }),

  showModalRestoreAll: () => set({ isModalVisibleRestoreAll: true }),
  hideModalRestoreAll: () => set({ isModalVisibleRestoreAll: false }),

  showModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: true }),
  hideModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: false }),
}));

export default useModalMerchantTrashed;
