import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalUserTrashed from "@/store/user/trashed/modal";
import useUserTrashedStore from "@/store/user/trashed/trashed";
import { RestoreUserTrashed } from "@/types/domain/request";

export default function useRestoreUser() {
  const {
    restoreUserId,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useModalUserTrashed();

  const {
    restoreUserTrashed,
    setLoadingRestoreUserTrashed,
    loadingRestoreUserTrashed,
    setErrorRestoreUserTrashed,
  } = useUserTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreUserTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: RestoreUserTrashed = {
        id: restoreUserId as number,
        toast: toast,
      };

      const result = await restoreUserTrashed(req);

      if (result) {
        toast({
          title: "Success",
          description: "User berhasil direstore",
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
      setErrorRestoreUserTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreUserTrashed(false);
      hideModalRestore();
    }
  };

  return {
    restoreUserId,
    handleSubmit,
    loadingRestoreUserTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  };
}
