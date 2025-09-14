import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalRoleTrashed from "@/store/role/trashed/modal";
import useRoleTrashedStore from "@/store/role/trashed/trashed";

export default function useDeletePermanentAllRole() {
  const {
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useModalRoleTrashed();

  const {
    deletePermanentAllRole,
    setLoadingDeletePermanentAllRoleTrashed,
    loadingDeletePermanentAllRoleTrashed,
    setErrorDeletePermanentAllRoleTrashed,
  } = useRoleTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentAllRoleTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await deletePermanentAllRole(toast);

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
          description: "Gagal menghapus Role. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Role";
      setErrorDeletePermanentAllRoleTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentAllRoleTrashed(false);
      hideModalDeletePermanentAll();
    }
  };

  return {
    handleSubmit,
    loadingDeletePermanentAllRoleTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  };
}
