export interface ModalCardTrashedStore {
  isModalVisibleRestore: boolean;
  isModalVisibleDeletePermanent: boolean;
  isModalVisibleRestoreAll: boolean;
  isModalVisibleDeletePermanentAll: boolean;

  restoreCardId: number | null;
  deletePermanentCardId: number | null;

  showModalRestore: (id: number) => void;
  hideModalRestore: () => void;

  showModalDeletePermanent: (id: number) => void;
  hideModalDeletePermanent: () => void;

  showModalRestoreAll: () => void;
  hideModalRestoreAll: () => void;

  showModalDeletePermanentAll: () => void;
  hideModalDeletePermanentAll: () => void;
}
