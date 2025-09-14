import { ModalMerchantStore } from "@/types/state/merchant";
import { create } from "zustand";

const useModalMerchant = create<ModalMerchantStore>((set) => ({
    isModalVisible: false,
    isModalVisibleEdit: false,
    isModalVisibleDelete: false,
    isModalVisibleImport: false,

    editMerchantId: null,
    deleteMerchantId: null,

   
    showModal: () => set({ isModalVisible: true }),
    hideModal: () => set({ isModalVisible: false }),

    showModalEdit: (id: number) => set({ isModalVisibleEdit: true, editMerchantId: id }),
    hideModalEdit: () => set({ isModalVisibleEdit: false, editMerchantId: null }),

    showModalDelete: (id: number) => set({ isModalVisibleDelete: true, deleteMerchantId: id }),
    hideModalDelete: () => set({ isModalVisibleDelete: false, deleteMerchantId: null }),

    showModalImport: () => set({ isModalVisibleImport: true }),
    hideModalImport: () => set({ isModalVisibleImport: false }),
}));

export default useModalMerchant;