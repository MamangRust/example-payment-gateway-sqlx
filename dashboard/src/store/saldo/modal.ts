import { ModalSaldoStore } from "@/types/state/saldo";
import { create } from "zustand";

const useModalSaldo = create<ModalSaldoStore>((set) => ({
    isModalVisible: false,
    isModalVisibleEdit: false,
    isModalVisibleDelete: false,
    isModalVisibleImport: false,

    editSaldoId: null,
    deleteSaldoId: null,

   
    showModal: () => set({ isModalVisible: true }),
    hideModal: () => set({ isModalVisible: false }),

    showModalEdit: (id: number) => set({ isModalVisibleEdit: true, editSaldoId: id }),
    hideModalEdit: () => set({ isModalVisibleEdit: false, editSaldoId: null }),

    showModalDelete: (id: number) => set({ isModalVisibleDelete: true, deleteSaldoId: id }),
    hideModalDelete: () => set({ isModalVisibleDelete: false, deleteSaldoId: null }),

    showModalImport: () => set({ isModalVisibleImport: true }),
    hideModalImport: () => set({ isModalVisibleImport: false }),
}));

export default useModalSaldo;