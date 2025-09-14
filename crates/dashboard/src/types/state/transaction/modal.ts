export interface ModalTransactionStore {
    isModalVisible: boolean;
    isModalVisibleEdit: boolean;
    isModalVisibleDelete: boolean;
    isModalVisibleImport: boolean;

    editTransactionId: number | null;
    deleteTransactionId: number | null;

    
    showModal: () => void;
    hideModal: () => void;

    showModalEdit: (id: number) => void;
    hideModalEdit: () => void;

    showModalDelete: (id: number) => void;
    hideModalDelete: () => void;

    showModalImport: () => void;
    hideModalImport: () => void;
}