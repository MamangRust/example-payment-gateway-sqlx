export interface ModalTransferStore {
    isModalVisible: boolean;
    isModalVisibleEdit: boolean;
    isModalVisibleDelete: boolean;
    isModalVisibleImport: boolean;

    editTransferId: number | null;
    deleteTransferId: number | null;

    
    showModal: () => void;
    hideModal: () => void;

    showModalEdit: (id: number) => void;
    hideModalEdit: () => void;

    showModalDelete: (id: number) => void;
    hideModalDelete: () => void;

    showModalImport: () => void;
    hideModalImport: () => void;
}