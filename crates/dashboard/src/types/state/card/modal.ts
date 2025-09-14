export interface ModalCardStore {
    isModalVisible: boolean;
    isModalVisibleEdit: boolean;
    isModalVisibleDelete: boolean;
    isModalVisibleImport: boolean;

    editCardId: number | null;
    deleteCardId: number | null;

    
    showModal: () => void;
    hideModal: () => void;

    showModalEdit: (id: number) => void;
    hideModalEdit: () => void;

    showModalDelete: (id: number) => void;
    hideModalDelete: () => void;

    showModalImport: () => void;
    hideModalImport: () => void;
}