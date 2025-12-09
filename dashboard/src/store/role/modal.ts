import { ModalRoleStore } from "@/types/state/role";
import { create } from "zustand";

const useModalRole = create<ModalRoleStore>((set) => ({
    isModalVisible: false,
    isModalVisibleEdit: false,
    isModalVisibleDelete: false,
    isModalVisibleImport: false,

    editRoleId: null,
    deleteRoleId: null,

   
    showModal: () => set({ isModalVisible: true }),
    hideModal: () => set({ isModalVisible: false }),

    showModalEdit: (id: number) => set({ isModalVisibleEdit: true, editRoleId: id }),
    hideModalEdit: () => set({ isModalVisibleEdit: false, editRoleId: null }),

    showModalDelete: (id: number) => set({ isModalVisibleDelete: true, deleteRoleId: id }),
    hideModalDelete: () => set({ isModalVisibleDelete: false, deleteRoleId: null }),

    showModalImport: () => set({ isModalVisibleImport: true }),
    hideModalImport: () => set({ isModalVisibleImport: false }),
}));

export default useModalRole;