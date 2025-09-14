import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";
import useModalTransaferTrashed from "@/store/transfer/trashed/modal";
import useTransferTrashedStore from "@/store/transfer/trashed/trashed";

export default function useRestoreAllTransfer() {
  const { isModalVisibleRestoreAll, showModalRestoreAll, hideModalRestoreAll } =
    useModalTransaferTrashed();

  const {
    restoreTransferAllTrashed,
    setLoadingRestoreAllTransferTrashed,
    loadingRestoreAllTransferTrashed,
    setErrorRestoreAllTransferTrashed,
  } = useTransferTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreAllTransferTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const result = await restoreTransferAllTrashed(toast);

      if (result) {
        toast({
          title: "Success",
          description: "Transfer berhasil dihapus",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal restore Transfer. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat restore Transfer";
      setErrorRestoreAllTransferTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreAllTransferTrashed(false);
      hideModalRestoreAll();
    }
  };

  return {
    handleSubmit,
    loadingRestoreAllTransferTrashed,
    isModalVisibleRestoreAll,
    showModalRestoreAll,
    hideModalRestoreAll,
  };
}
