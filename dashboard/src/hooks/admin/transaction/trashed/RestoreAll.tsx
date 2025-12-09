import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalTransactionTrashed from "@/store/transaction/trashed/modal";
import useTransactionTrashedStore from "@/store/transaction/trashed/trashed";

export default function useRestoreAllTransaction() {
  const { isModalVisibleRestoreAll, showModalRestoreAll, hideModalRestoreAll } =
    useModalTransactionTrashed();

  const {
    restoreTransactionAllTrashed,
    setLoadingRestoreAllTransactionTrashed,
    loadingRestoreAllTransactionTrashed,
    setErrorRestoreAllTransactionTrashed,
  } = useTransactionTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreAllTransactionTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await restoreTransactionAllTrashed(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Transaction berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal restore Transaction. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat restore Transaction";
      setErrorRestoreAllTransactionTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreAllTransactionTrashed(false);
      hideModalRestoreAll();
    }
  };

  return {
    handleSubmit,
    loadingRestoreAllTransactionTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  };
}
