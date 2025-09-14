export interface ModalWithdrawTrashedStore {
  isModalVisibleRestore: boolean;
  isModalVisibleDeletePermanent: boolean;
  isModalVisibleRestoreAll: boolean;
  isModalVisibleDeletePermanentAll: boolean;

  restoreWithdrawId: number | null;
  deletePermanentWithdrawId: number | null;

  showModalRestore: (id: number) => void;
  hideModalRestore: () => void;

  showModalDeletePermanent: (id: number) => void;
  hideModalDeletePermanent: () => void;

  showModalRestoreAll: () => void;
  hideModalRestoreAll: () => void;

  showModalDeletePermanentAll: () => void;
  hideModalDeletePermanentAll: () => void;
}
