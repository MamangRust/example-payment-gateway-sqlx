import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalUserTrashed from "@/store/user/trashed/modal";
import useUserTrashedStore from "@/store/user/trashed/trashed";

export default function useRestoreAllUser() {
  const { isModalVisibleRestoreAll, showModalRestoreAll, hideModalRestoreAll } =
    useModalUserTrashed();

  const {
    restoreUserAllTrashed,
    setLoadingRestoreAllUserTrashed,
    loadingRestoreAllUserTrashed,
    setErrorRestoreAllUserTrashed,
  } = useUserTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreAllUserTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await restoreUserAllTrashed(toast);

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
          description: "Gagal restore User. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat restore User";
      setErrorRestoreAllUserTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreAllUserTrashed(false);
      hideModalRestoreAll();
    }
  };

  return {
    handleSubmit,
    loadingRestoreAllUserTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  };
}
