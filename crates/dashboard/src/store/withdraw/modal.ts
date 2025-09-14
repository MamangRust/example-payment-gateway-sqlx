import { ModalWithdrawStore } from "@/types/state/withdraw";
import { create } from "zustand";

const useModalWithdraw = create<ModalWithdrawStore>((set) => ({
    isModalVisible: false,
    isModalVisibleEdit: false,
    isModalVisibleDelete: false,
    isModalVisibleImport: false,

    editWithdrawId: null,
    deleteWithdrawId: null,

   
    showModal: () => set({ isModalVisible: true }),
    hideModal: () => set({ isModalVisible: false }),

    showModalEdit: (id: number) => set({ isModalVisibleEdit: true, editWithdrawId: id }),
    hideModalEdit: () => set({ isModalVisibleEdit: false, editWithdrawId: null }),

    showModalDelete: (id: number) => set({ isModalVisibleDelete: true, deleteWithdrawId: id }),
    hideModalDelete: () => set({ isModalVisibleDelete: false, deleteWithdrawId: null }),

    showModalImport: () => set({ isModalVisibleImport: true }),
    hideModalImport: () => set({ isModalVisibleImport: false }),
}));

export default useModalWithdraw;