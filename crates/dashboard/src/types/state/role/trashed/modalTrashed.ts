export interface ModalRoleTrashedStore {
    isModalVisibleRestore: boolean;
    isModalVisibleDeletePermanent: boolean;
    isModalVisibleRestoreAll: boolean;
    isModalVisibleDeletePermanentAll: boolean;
  
    restoreRoleId: number | null;
    deletePermanentRoleId: number | null;
  
    showModalRestore: (id: number) => void;
    hideModalRestore: () => void;
  
    showModalDeletePermanent: (id: number) => void;
    hideModalDeletePermanent: () => void;
  
    showModalRestoreAll: () => void;
    hideModalRestoreAll: () => void;
  
    showModalDeletePermanentAll: () => void;
    hideModalDeletePermanentAll: () => void;
  }
  