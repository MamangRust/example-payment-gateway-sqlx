export interface ModalUserStore {
  isModalVisible: boolean;
  isModalVisibleEdit: boolean;
  isModalVisibleDelete: boolean;
  isModalVisibleImport: boolean;

  editUserId: number | null;
  deleteUserId: number | null;

  showModal: () => void;
  hideModal: () => void;

  showModalEdit: (id: number) => void;
  hideModalEdit: () => void;

  showModalDelete: (id: number) => void;
  hideModalDelete: () => void;

  showModalImport: () => void;
  hideModalImport: () => void;
}
