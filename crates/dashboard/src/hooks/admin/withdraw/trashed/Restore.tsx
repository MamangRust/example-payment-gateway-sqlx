import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalWithdrawTrashed from "@/store/withdraw/trashed/modal";
import useWithdrawTrashedStore from "@/store/withdraw/trashed/trashed";
import { RestoreWithdrawTrashed } from "@/types/domain/request";

export default function useRestoreWithdraw() {
  const {
    restoreWithdrawId,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useModalWithdrawTrashed();

  const {
    restoreWithdrawTrashed,
    setLoadingRestoreWithdrawTrashed,
    loadingRestoreWithdrawTrashed,
    setErrorRestoreWithdrawTrashed,
  } = useWithdrawTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreWithdrawTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: RestoreWithdrawTrashed = {
        id: restoreWithdrawId as number,
        toast: toast,
      };

      const result = await restoreWithdrawTrashed(req);

      if (result) {
        toast({
          title: "Success",
          description: "Withdraw berhasil direstore",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Withdraw. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Withdraw";
      setErrorRestoreWithdrawTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreWithdrawTrashed(false);
      hideModalRestore();
    }
  };

  return {
    restoreWithdrawId,
    handleSubmit,
    loadingRestoreWithdrawTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  };
}
