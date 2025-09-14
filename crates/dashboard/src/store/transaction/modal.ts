import { ModalTransactionStore } from "@/types/state/transaction";
import { create } from "zustand";

const useModalTransaction = create<ModalTransactionStore>((set) => ({
    isModalVisible: false,
    isModalVisibleEdit: false,
    isModalVisibleDelete: false,
    isModalVisibleImport: false,

    editTransactionId: null,
    deleteTransactionId: null,

   
    showModal: () => set({ isModalVisible: true }),
    hideModal: () => set({ isModalVisible: false }),

    showModalEdit: (id: number) => set({ isModalVisibleEdit: true, editTransactionId: id }),
    hideModalEdit: () => set({ isModalVisibleEdit: false, editTransactionId: null }),

    showModalDelete: (id: number) => set({ isModalVisibleDelete: true, deleteTransactionId: id }),
    hideModalDelete: () => set({ isModalVisibleDelete: false, deleteTransactionId: null }),

    showModalImport: () => set({ isModalVisibleImport: true }),
    hideModalImport: () => set({ isModalVisibleImport: false }),
}));

export default useModalTransaction;