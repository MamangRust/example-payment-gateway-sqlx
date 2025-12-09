import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalRoleTrashed from "@/store/role/trashed/modal";
import useRoleTrashedStore from "@/store/role/trashed/trashed";

export default function RoleStoreAllRole() {
  const { isModalVisibleRestoreAll, showModalRestoreAll, hideModalRestoreAll } =
    useModalRoleTrashed();

  const {
    restoreRoleAllTrashed,
    setLoadingRestoreAllRoleTrashed,
    loadingRestoreAllRoleTrashed,
    setErrorRestoreAllRoleTrashed,
  } = useRoleTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreAllRoleTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await restoreRoleAllTrashed(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Role berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal restore Role. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat restore Role";
      setErrorRestoreAllRoleTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreAllRoleTrashed(false);
      hideModalRestoreAll();
    }
  };

  return {
    handleSubmit,
    loadingRestoreAllRoleTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  };
}
