import { useNavigate } from "react-router-dom";
import { useToast } from "@/hooks/use-toast";

import { RestoreTransferTrashed } from "@/types/domain/request";
import useModalTransaferTrashed from "@/store/transfer/trashed/modal";
import useTransferTrashedStore from "@/store/transfer/trashed/trashed";

export default function useRestoreTransfer() {
  const {
    restoreTransferId,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  } = useModalTransaferTrashed();

  const {
    restoreTransferTrashed,
    setLoadingRestoreTransferTrashed,
    loadingRestoreTransferTrashed,
    setErrorRestoreTransferTrashed,
  } = useTransferTrashedStore();

  const navigate = useNavigate();
  const { toast } = useToast();

  const handleSubmit = async () => {
    setLoadingRestoreTransferTrashed(true);

    try {
      await new Promise((resolve) => setTimeout(resolve, 1200));

      const req: RestoreTransferTrashed = {
        id: restoreTransferId as number,
        toast: toast,
      };

      const result = await restoreTransferTrashed(req);

      if (result) {
        toast({
          title: "Success",
          description: "Transfer berhasil direstore",
          variant: "default",
        });

        navigate(window.location.pathname);
      } else {
        toast({
          title: "Error",
          description: "Gagal menghapus Transfer. Silakan coba lagi.",
          variant: "destructive",
        });
      }
    } catch (error: any) {
      const errorMessage =
        error?.message || "Terjadi kesalahan saat menghapus Transfer";
      setErrorRestoreTransferTrashed(errorMessage);
      toast({
        title: "Error",
        description: errorMessage,
        variant: "destructive",
      });
    } finally {
      setLoadingRestoreTransferTrashed(false);
      hideModalRestore();
    }
  };

  return {
    restoreTransferId,
    handleSubmit,
    loadingRestoreTransferTrashed,
    isModalVisibleRestore,
    showModalRestore,
    hideModalRestore,
  };
}
