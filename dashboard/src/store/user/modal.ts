import { ModalUserStore } from "@/types/state/user";
import { create } from "zustand";

const useModalUser = create<ModalUserStore>((set) => ({
    isModalVisible: false,
    isModalVisibleEdit: false,
    isModalVisibleDelete: false,
    isModalVisibleImport: false,

    editUserId: null,
    deleteUserId: null,

   
    showModal: () => set({ isModalVisible: true }),
    hideModal: () => set({ isModalVisible: false }),

    showModalEdit: (id: number) => set({ isModalVisibleEdit: true, editUserId: id }),
    hideModalEdit: () => set({ isModalVisibleEdit: false, editUserId: null }),

    showModalDelete: (id: number) => set({ isModalVisibleDelete: true, deleteUserId: id }),
    hideModalDelete: () => set({ isModalVisibleDelete: false, deleteUserId: null }),

    showModalImport: () => set({ isModalVisibleImport: true }),
    hideModalImport: () => set({ isModalVisibleImport: false }),
}));

export default useModalUser;