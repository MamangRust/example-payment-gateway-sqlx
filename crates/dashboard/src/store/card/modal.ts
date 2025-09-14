import { ModalCardStore } from "@/types/state/card";
import { create } from "zustand";

const useModalCard = create<ModalCardStore>((set) => ({
    isModalVisible: false,
    isModalVisibleEdit: false,
    isModalVisibleDelete: false,
    isModalVisibleImport: false,

    editCardId: null,
    deleteCardId: null,

    showModal: () => set({ isModalVisible: true }),
    hideModal: () => set({ isModalVisible: false }),

    showModalEdit: (id: number) => set({ isModalVisibleEdit: true, editCardId: id }),
    hideModalEdit: () => set({ isModalVisibleEdit: false, editCardId: null }),

    showModalDelete: (id: number) => set({ isModalVisibleDelete: true, deleteCardId: id }),
    hideModalDelete: () => set({ isModalVisibleDelete: false, deleteCardId: null }),

    showModalImport: () => set({ isModalVisibleImport: true }),
    hideModalImport: () => set({ isModalVisibleImport: false }),
}));

export default useModalCard;