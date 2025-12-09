import { create } from "zustand";
import { ModalRoleTrashedStore } from "@/types/state";

const useModalRoleTrashed = create<ModalRoleTrashedStore>((set) => ({
  isModalVisibleRestore: false,
  isModalVisibleDeletePermanent: false,
  isModalVisibleRestoreAll: false,
  isModalVisibleDeletePermanentAll: false,

  restoreRoleId: null,
  deletePermanentRoleId: null,

  showModalRestore: (id: number) =>
    set({ isModalVisibleRestore: true, restoreRoleId: id }),
  hideModalRestore: () =>
    set({ isModalVisibleRestore: false, restoreRoleId: null }),

  showModalDeletePermanent: (id: number) =>
    set({
      isModalVisibleDeletePermanent: true,
      deletePermanentRoleId: id,
    }),
  hideModalDeletePermanent: () =>
    set({
      isModalVisibleDeletePermanent: false,
      deletePermanentRoleId: null,
    }),

  showModalRestoreAll: () => set({ isModalVisibleRestoreAll: true }),
  hideModalRestoreAll: () => set({ isModalVisibleRestoreAll: false }),

  showModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: true }),
  hideModalDeletePermanentAll: () =>
    set({ isModalVisibleDeletePermanentAll: false }),
}));

export default useModalRoleTrashed;
