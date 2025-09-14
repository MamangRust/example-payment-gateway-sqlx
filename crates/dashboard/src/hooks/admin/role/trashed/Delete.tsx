import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalRoleTrashed from "@/store/role/trashed/modal";
import useRoleTrashedStore from "@/store/role/trashed/trashed";
import { DeletePermanentRole } from "@/types/domain/request";

export default function useDeletePermanentRole() {
  const {
    deletePermanentRoleId,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useModalRoleTrashed();

  const {
    deletePermanentRole,
    setLoadingDeletePermanentRoleTrashed,
    loadingDeletePermanentRoleTrashed,
    setErrorDeletePermanentRoleTrashed,
  } = useRoleTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentRoleTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: DeletePermanentRole = {
        id: deletePermanentRoleId as number,
        toast: toast,
      };

      const result = await deletePermanentRole(req);

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
      setErrorDeletePermanentRoleTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentRoleTrashed(false);
      hideModalDeletePermanent();
    }
  };

  return {
    deletePermanentRoleId,
    handleSubmit,
    loadingDeletePermanentRoleTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  };
}
