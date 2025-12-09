import { ModalTransferStore } from "@/types/state/transfer";
import { create } from "zustand";

const useModalTransfer = create<ModalTransferStore>((set) => ({
    isModalVisible: false,
    isModalVisibleEdit: false,
    isModalVisibleDelete: false,
    isModalVisibleImport: false,

    editTransferId: null,
    deleteTransferId: null,

   
    showModal: () => set({ isModalVisible: true }),
    hideModal: () => set({ isModalVisible: false }),

    showModalEdit: (id: number) => set({ isModalVisibleEdit: true, editTransferId: id }),
    hideModalEdit: () => set({ isModalVisibleEdit: false, editTransferId: null }),

    showModalDelete: (id: number) => set({ isModalVisibleDelete: true, deleteTransferId: id }),
    hideModalDelete: () => set({ isModalVisibleDelete: false, deleteTransferId: null }),

    showModalImport: () => set({ isModalVisibleImport: true }),
    hideModalImport: () => set({ isModalVisibleImport: false }),
}));

export default useModalTransfer;