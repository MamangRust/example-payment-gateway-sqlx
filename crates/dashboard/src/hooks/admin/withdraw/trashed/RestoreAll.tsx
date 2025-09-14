import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalWithdrawTrashed from "@/store/withdraw/trashed/modal";
import useWithdrawTrashedStore from "@/store/withdraw/trashed/trashed";

export default function useRestoreAllWithdraw() {
  const { isModalVisibleRestoreAll, showModalRestoreAll, hideModalRestoreAll } =
    useModalWithdrawTrashed();

  const {
    restoreWithdrawAllTrashed,
    setLoadingRestoreAllWithdrawTrashed,
    loadingRestoreAllWithdrawTrashed,
    setErrorRestoreAllWithdrawTrashed,
  } = useWithdrawTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreAllWithdrawTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await restoreWithdrawAllTrashed(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Withdraw berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal restore Withdraw. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat restore Withdraw";
      setErrorRestoreAllWithdrawTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreAllWithdrawTrashed(false);
      hideModalRestoreAll();
    }
  };

  return {
    handleSubmit,
    loadingRestoreAllWithdrawTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  };
}
