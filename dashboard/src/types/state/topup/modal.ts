export interface ModalTopupStore {
    isModalVisible: boolean;
    isModalVisibleEdit: boolean;
    isModalVisibleDelete: boolean;
    isModalVisibleImport: boolean;

    editTopupId: number | null;
    deleteTopupId: number | null;

    
    showModal: () => void;
    hideModal: () => void;

    showModalEdit: (id: number) => void;
    hideModalEdit: () => void;

    showModalDelete: (id: number) => void;
    hideModalDelete: () => void;

    showModalImport: () => void;
    hideModalImport: () => void;
}