export interface ModalSaldoTrashedStore {
  isModalVisibleRestore: boolean;
  isModalVisibleDeletePermanent: boolean;
  isModalVisibleRestoreAll: boolean;
  isModalVisibleDeletePermanentAll: boolean;

  restoreSaldoId: number | null;
  deletePermanentSaldoId: number | null;

  showModalRestore: (id: number) => void;
  hideModalRestore: () => void;

  showModalDeletePermanent: (id: number) => void;
  hideModalDeletePermanent: () => void;

  showModalRestoreAll: () => void;
  hideModalRestoreAll: () => void;

  showModalDeletePermanentAll: () => void;
  hideModalDeletePermanentAll: () => void;
}
