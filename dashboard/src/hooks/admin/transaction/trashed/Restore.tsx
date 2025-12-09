import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalTransactionTrashed from "@/store/transaction/trashed/modal";
import useTransactionTrashedStore from "@/store/transaction/trashed/trashed";
import { RestoreTransactionTrashed } from "@/types/domain/request";

export default function useRestoreTransaction() {
  const {
    restoreTransactionId,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useModalTransactionTrashed();

  const {
    restoreTransactionTrashed,
    setLoadingRestoreTransactionTrashed,
    loadingRestoreTransactionTrashed,
    setErrorRestoreTransactionTrashed,
  } = useTransactionTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreTransactionTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: RestoreTransactionTrashed = {
        id: restoreTransactionId as number,
        toast: toast,
      };

      const result = await restoreTransactionTrashed(req);

      if (result) {
        toast({
          title: "Success",
          description: "Transaction berhasil direstore",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Transaction. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Transaction";
      setErrorRestoreTransactionTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreTransactionTrashed(false);
      hideModalRestore();
    }
  };

  return {
    restoreTransactionId,
    handleSubmit,
    loadingRestoreTransactionTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  };
}
