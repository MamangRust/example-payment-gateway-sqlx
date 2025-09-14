export interface ModalSaldoStore {
    isModalVisible: boolean;
    isModalVisibleEdit: boolean;
    isModalVisibleDelete: boolean;
    isModalVisibleImport: boolean;

    editSaldoId: number | null;
    deleteSaldoId: number | null;

    
    showModal: () => void;
    hideModal: () => void;

    showModalEdit: (id: number) => void;
    hideModalEdit: () => void;

    showModalDelete: (id: number) => void;
    hideModalDelete: () => void;

    showModalImport: () => void;
    hideModalImport: () => void;
}