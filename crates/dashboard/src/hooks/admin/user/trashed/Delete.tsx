import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalUserTrashed from "@/store/user/trashed/modal";
import useUserTrashedStore from "@/store/user/trashed/trashed";
import { DeletePermanentUser } from "@/types/domain/request";

export default function useDeletePermanentUser() {
  const {
    deletePermanentUserId,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  } = useModalUserTrashed();

  const {
    deletePermanentUser,
    setLoadingDeletePermanentUserTrashed,
    loadingDeletePermanentUserTrashed,
    setErrorDeletePermanentUserTrashed,
  } = useUserTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentUserTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: DeletePermanentUser = {
        id: deletePermanentUserId as number,
        toast: toast,
      };

      const result = await deletePermanentUser(req);

      if (result) {
        toast({
          title: "Success",
          description: "User berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus User. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus User";
      setErrorDeletePermanentUserTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentUserTrashed(false);
      hideModalDeletePermanent();
    }
  };

  return {
    deletePermanentUserId,
    handleSubmit,
    loadingDeletePermanentUserTrashed,
    isModalVisibleDeletePermanent,
    showModalDeletePermanent,
    hideModalDeletePermanent,
  };
}
