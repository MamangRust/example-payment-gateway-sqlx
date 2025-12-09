import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalUserTrashed from "@/store/user/trashed/modal";
import useUserTrashedStore from "@/store/user/trashed/trashed";

export default function useDeletePermanentAllUser() {
  const {
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  } = useModalUserTrashed();

  const {
    deletePermanentAllUser,
    setLoadingDeletePermanentAllUserTrashed,
    loadingDeletePermanentAllUserTrashed,
    setErrorDeletePermanentAllUserTrashed,
  } = useUserTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingDeletePermanentAllUserTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await deletePermanentAllUser(toast);

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
      setErrorDeletePermanentAllUserTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingDeletePermanentAllUserTrashed(false);
      hideModalDeletePermanentAll();
    }
  };

  return {
    handleSubmit,
    loadingDeletePermanentAllUserTrashed,
    isModalVisibleDeletePermanentAll,
    showModalDeletePermanentAll,
    hideModalDeletePermanentAll,
  };
}
