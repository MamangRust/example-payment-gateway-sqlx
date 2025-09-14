import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalRoleTrashed from "@/store/role/trashed/modal";
import useRoleTrashedStore from "@/store/role/trashed/trashed";
import { RestoreRoleTrashed } from "@/types/domain/request";

export default function useRoleRestore() {
  const {
    restoreRoleId,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useModalRoleTrashed();

  const {
    restoreRoleTrashed,
    setLoadingRestoreRoleTrashed,
    loadingRestoreRoleTrashed,
    setErrorRestoreRoleTrashed,
  } = useRoleTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreRoleTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: RestoreRoleTrashed = {
        id: restoreRoleId as number,
        toast: toast,
      };

      const result = await restoreRoleTrashed(req);

      if (result) {
        toast({
          title: "Success",
          description: "Role berhasil direstore",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Role. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Role";
      setErrorRestoreRoleTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreRoleTrashed(false);
      hideModalRestore();
    }
  };

  return {
    restoreRoleId,
    handleSubmit,
    loadingRestoreRoleTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  };
}
