export interface ModalUserTrashedStore {
  isModalVisibleRestore: boolean;
  isModalVisibleDeletePermanent: boolean;
  isModalVisibleRestoreAll: boolean;
  isModalVisibleDeletePermanentAll: boolean;

  restoreUserId: number | null;
  deletePermanentUserId: number | null;

  showModalRestore: (id: number) => void;
  hideModalRestore: () => void;

  showModalDeletePermanent: (id: number) => void;
  hideModalDeletePermanent: () => void;

  showModalRestoreAll: () => void;
  hideModalRestoreAll: () => void;

  showModalDeletePermanentAll: () => void;
  hideModalDeletePermanentAll: () => void;
}
