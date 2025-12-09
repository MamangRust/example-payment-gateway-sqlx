export interface ModalTopupTrashedStore {
  isModalVisibleRestore: boolean;
  isModalVisibleDeletePermanent: boolean;
  isModalVisibleRestoreAll: boolean;
  isModalVisibleDeletePermanentAll: boolean;

  restoreTopupId: number | null;
  deletePermanentTopupId: number | null;

  showModalRestore: (id: number) => void;
  hideModalRestore: () => void;

  showModalDeletePermanent: (id: number) => void;
  hideModalDeletePermanent: () => void;

  showModalRestoreAll: () => void;
  hideModalRestoreAll: () => void;

  showModalDeletePermanentAll: () => void;
  hideModalDeletePermanentAll: () => void;
}
