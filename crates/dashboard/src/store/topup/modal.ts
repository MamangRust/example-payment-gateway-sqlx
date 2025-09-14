import { ModalTopupStore } from "@/types/state/topup";
import { create } from "zustand";

const useModalTopup = create<ModalTopupStore>((set) => ({
    isModalVisible: false,
    isModalVisibleEdit: false,
    isModalVisibleDelete: false,
    isModalVisibleImport: false,

    editTopupId: null,
    deleteTopupId: null,

   
    showModal: () => set({ isModalVisible: true }),
    hideModal: () => set({ isModalVisible: false }),

    showModalEdit: (id: number) => set({ isModalVisibleEdit: true, editTopupId: id }),
    hideModalEdit: () => set({ isModalVisibleEdit: false, editTopupId: null }),

    showModalDelete: (id: number) => set({ isModalVisibleDelete: true, deleteTopupId: id }),
    hideModalDelete: () => set({ isModalVisibleDelete: false, deleteTopupId: null }),

    showModalImport: () => set({ isModalVisibleImport: true }),
    hideModalImport: () => set({ isModalVisibleImport: false }),
}));

export default useModalTopup;